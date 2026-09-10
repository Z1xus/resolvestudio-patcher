#[cfg(not(any(
    all(
        any(target_os = "linux", target_os = "windows"),
        target_arch = "x86_64"
    ),
    all(
        target_os = "macos",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )
)))]
compile_error!("supported hosts: linux/windows x86-64, macos x86-64/arm64");

mod log;

use resolvestudio_patcher::{
    binary, bundle, engine, profiles,
    transaction::{self, Transaction},
};
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    process::ExitCode,
};

const HELP: &str = "resolvestudio-patcher

usage:
  resolvestudio-patcher check <path>
  resolvestudio-patcher patch <path>
  resolvestudio-patcher restore <path>
  resolvestudio-patcher cleanup <path>

--verbose, -v: show hashes, offsets and bytes
--try-profile <id>: try a profile outside its version range (check or patch)
macos: <path> can also be an .app bundle
backups: <path>.backups/ (beside the .app for bundled executables)";

fn confirm_cleanup(bytes: u64) -> Result<bool, String> {
    let mut size = bytes as f64;
    let mut unit = "bytes";
    for next in ["KiB", "MiB", "GiB", "TiB"] {
        if size < 1024.0 {
            break;
        }
        size /= 1024.0;
        unit = next;
    }
    print!(
        "this will remove your backups, freeing approximately {size:.2} {unit} of space. \
         you will not be able to go back to that version. do you wish to continue? y/n: "
    );
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .map_err(|e| e.to_string())?;
    Ok(answer.trim().eq_ignore_ascii_case("y"))
}

fn run() -> Result<(), String> {
    let mut args: Vec<_> = env::args_os().skip(1).collect();
    let verbose = args.iter().any(|arg| arg == "--verbose" || arg == "-v");
    args.retain(|arg| arg != "--verbose" && arg != "-v");
    let mut try_profile = None;
    if let Some(index) = args.iter().position(|arg| arg == "--try-profile") {
        let id = args
            .get(index + 1)
            .ok_or("--try-profile requires a profile id")?;
        try_profile = Some(id.to_str().ok_or("invalid profile id")?.to_owned());
        args.drain(index..index + 2);
    }
    if args
        .first()
        .is_some_and(|arg| arg == "--help" || arg == "-h")
    {
        println!("{HELP}");
        let mut ids = Vec::new();
        for profile in profiles::ALL {
            if !ids.contains(&profile.id) {
                ids.push(profile.id);
            }
        }
        println!("available profiles: {}", ids.join(", "));
        return Ok(());
    }
    if args.len() == 1 && args[0] == "--version" {
        log::info(format!(
            "resolvestudio-patcher {}",
            env!("CARGO_PKG_VERSION")
        ));
        return Ok(());
    }
    let command = match args.first() {
        Some(arg) => arg.to_str().ok_or("invalid command")?,
        None => return Err("command and executable path required, use --help".into()),
    };
    match command {
        "check" | "patch" | "restore" | "cleanup" if args.len() == 1 => {
            return Err("executable path required, use --help".into());
        }
        "check" | "patch" | "restore" | "cleanup" if args.len() == 2 => {}
        _ => return Err("invalid arguments, use --help".into()),
    }
    if try_profile.is_some() && !matches!(command, "check" | "patch") {
        return Err("--try-profile requires check or patch".into());
    }
    let path = PathBuf::from(&args[1]);
    if path.as_os_str().is_empty() {
        return Err("executable path required, use --help".into());
    }
    let path = bundle::executable(&path)?;
    if command == "cleanup" {
        let mut transaction = Transaction::open(&path)?;
        log::info(format!("backup folder: {}", transaction.backups.display()));
        if transaction.cleanup(confirm_cleanup)?.is_some() {
            log::success("cleanup complete");
        } else {
            log::info("cleanup cancelled");
        }
        return Ok(());
    }
    if command == "restore" {
        log::info(format!("restoring {}", path.display()));
        let mut transaction = Transaction::open(&path)?;
        if transaction.restore()? {
            log::success("restored");
        } else {
            log::warning("already restored");
        }
        return Ok(());
    }
    let mut transaction = if command == "patch" {
        Some(Transaction::open(&path)?)
    } else {
        None
    };
    log::info(format!("reading {}", path.display()));
    let mut data = match &mut transaction {
        Some(transaction) => transaction.read()?,
        None => fs::read(&path)
            .map_err(|e| format!("cannot read executable: {}", e.to_string().to_lowercase()))?,
    };
    let slices = binary::inspect(&data)?;
    let platform = slices[0].target.platform;
    if command == "patch" && !platform.is_native() {
        return Err("run patch on the executable's operating system".into());
    }
    let fingerprint = engine::hash(&data);
    if verbose {
        log::info(format!("sha256: {fingerprint}"));
    }
    if let Some(id) = &try_profile {
        log::warning(format!("trying profile: {id}, version range ignored"));
    }
    let mut plans = Vec::new();
    let mut image_version = None;
    for slice in &slices {
        let bytes = &data[slice.offset..slice.offset + slice.size];
        let slice_hash = if slice.offset == 0 && slice.size == data.len() {
            fingerprint.clone()
        } else {
            engine::hash(bytes)
        };
        let version = engine::identify(bytes, &slice_hash, profiles::ALL).map_err(|e| {
            log::unknown(&e);
            format!("cannot identify {} build", slice.target.architecture)
        })?;
        if image_version.is_some_and(|previous| previous != version) {
            return Err("conflicting versions across executable slices".into());
        }
        image_version = Some(version);
        let plan = engine::plan(
            bytes,
            &slice.segments,
            version,
            &slice_hash,
            profiles::ALL,
            try_profile.as_deref(),
            slice.target,
        )?;
        if plan.build.is_some() {
            log::success(format!(
                "known build: {version} ({})",
                slice.target.architecture
            ));
        } else {
            log::warning(format!(
                "untested build: {version} ({}), signatures passed",
                slice.target.architecture
            ));
        }
        if verbose {
            log::info(format!(
                "profile: {} ({})",
                plan.profile.id, slice.target.architecture
            ));
            for change in &plan.changes {
                log::info(format!(
                    "{}: offset {:#x}, address {:#x}, {:02x?} -> {:02x?}",
                    change.name,
                    slice.offset + change.location.offset,
                    change.location.address,
                    change.before,
                    change.after
                ));
            }
        }
        plans.push(plan);
    }
    if plans
        .iter()
        .any(|plan| plan.already_patched != plans[0].already_patched)
    {
        return Err("partially patched executable slices".into());
    }
    if plans[0].already_patched {
        log::warning("already patched");
        return Ok(());
    }
    if let Some(transaction) = &mut transaction {
        if platform == binary::Platform::Macos {
            log::info("preserving entitlements and creating ad hoc code signatures");
        }
        for (slice, plan) in slices.iter().zip(&plans) {
            plan.apply(&mut data[slice.offset..slice.offset + slice.size])?;
        }
        let output_hash = engine::hash(&data);
        log::info("backing up and patching");
        if let Err(error) = transaction.patch(&data, &fingerprint, &output_hash) {
            log::warning(format!("backup folder: {}", transaction.backups.display()));
            return Err(error);
        }
        log::success("patched");
        log::info(format!("backup: {}", transaction.backups.display()));
        if verbose {
            log::info(format!("sha256: {output_hash}"));
        }
    } else {
        if platform.is_native()
            && let Err(message) = transaction::ensure_idle(&path)
        {
            log::warning(message);
        }
        log::success("check complete");
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            log::error(&error);
            if error.contains("permission denied") {
                log::warning(if cfg!(windows) {
                    "use an account with write access, or run as administrator"
                } else {
                    "use an account with write access, or sudo if required"
                });
            }
            ExitCode::FAILURE
        }
    }
}
