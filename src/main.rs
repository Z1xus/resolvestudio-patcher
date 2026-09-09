#[cfg(not(all(
    any(target_os = "linux", target_os = "windows"),
    target_arch = "x86_64"
)))]
compile_error!("resolvestudio-patcher supports linux and windows x86-64 only");

mod log;

use resolvestudio_patcher::{
    binary, engine, profiles,
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
backups: <path>.backups/";

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
        println!(
            "available profiles: {}",
            profiles::ALL
                .iter()
                .map(|profile| profile.id)
                .collect::<Vec<_>>()
                .join(", ")
        );
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
    let (platform, segments) = binary::inspect(&data)?;
    if command == "patch" && !platform.is_native() {
        return Err("run patch on the executable's operating system".into());
    }
    let fingerprint = engine::hash(&data);
    if verbose {
        log::info(format!("sha256: {fingerprint}"));
    }
    let version = engine::identify(&data, &fingerprint, profiles::ALL).map_err(|e| {
        log::unknown(&e);
        "cannot identify build".to_string()
    })?;
    if let Some(id) = &try_profile {
        log::warning(format!("trying profile: {id}, version range ignored"));
    }
    let plan = engine::plan(
        &data,
        &segments,
        version,
        &fingerprint,
        profiles::ALL,
        try_profile.as_deref(),
        platform,
    )?;
    if plan.build.is_some() {
        log::success(format!("tested build: {version}"));
    } else {
        log::warning(format!("untested build: {version}, signatures passed"));
    }
    if verbose {
        log::info(format!("profile: {}", plan.profile.id));
        for change in &plan.changes {
            log::info(format!(
                "{}: offset {:#x}, address {:#x}, {:02x?} -> {:02x?}",
                change.name,
                change.location.offset,
                change.location.address,
                change.before,
                change.after
            ));
        }
    }
    if plan.already_patched {
        log::warning("already patched");
        return Ok(());
    }
    if let Some(transaction) = &mut transaction {
        let output_hash = plan.apply(&mut data)?;
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
