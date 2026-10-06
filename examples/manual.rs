use resolvestudio_patcher::{
    binary::{Architecture, Platform},
    profile::{Action, Anchor, Profile},
    profiles,
};
use std::fs;

const LINUX: &str = "/opt/resolve/bin/resolve";
const MACOS: &str = "\"/Applications/DaVinci Resolve/DaVinci Resolve.app/Contents/MacOS/Resolve\"";
const MACOS_BACKUP: &str = "\"/Applications/DaVinci Resolve/Resolve.bak\"";
const WINDOWS: &str = r"C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe";

fn regex(signature: &str) -> String {
    signature
        .split_whitespace()
        .map(|token| match token {
            "?" | "??" => ".".into(),
            byte => format!("\\x{byte}"),
        })
        .collect()
}

fn width(signature: &str) -> usize {
    signature.split_whitespace().count()
}

fn code(action: &Action) -> (Vec<u8>, Vec<(usize, &Anchor)>) {
    match action {
        Action::Bytes(bytes) => (bytes.to_vec(), Vec::new()),
        Action::Code { bytes, calls } => (
            bytes.to_vec(),
            calls
                .iter()
                .map(|call| (call.offset, &call.target))
                .collect(),
        ),
        Action::Jump { target, .. } => (vec![0xe9, 0, 0, 0, 0], vec![(0, target)]),
        Action::Arm64Branch { .. } => unreachable!(),
    }
}

fn perl(profile: &Profile, path: &str) -> String {
    let at = |anchor: &Anchor| format!("at(qr/{}/s) + {}", regex(anchor.signature), anchor.offset);
    let mut script = String::from(
        "sudo perl -e '
open F, \"+<:raw\", $ARGV[0] or die \"$!\\n\";
$d = $o = do { local $/; <F> };
sub at { my @m; push @m, $-[0] while $d =~ /(?=$_[0])/g; @m == 1 ? $m[0] : die \"signature not found\\n\" }
",
    );
    for patch in profile.patches {
        let width = width(patch.expected);
        script += &format!(
            "$p = {};\nsubstr($d, $p, {width}) =~ /\\A{}\\z/s or die \"unexpected bytes\\n\";\n",
            at(&patch.anchor),
            regex(patch.expected)
        );
        if let Action::Arm64Branch { target, link, .. } = &patch.action {
            let opcode = if *link { "0x94000000" } else { "0x14000000" };
            script += &format!(
                "substr($o, $p, 4) = pack(\"V\", {opcode} | ({} - $p) / 4 & 0x3ffffff);\n",
                at(target)
            );
            continue;
        }
        let (bytes, calls) = code(&patch.action);
        let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        script += &format!("$c = pack(\"H*\", \"{hex}\");\n");
        for (offset, target) in calls {
            script += &format!(
                "substr($c, {}, 4) = pack(\"l<\", {} - $p - {});\n",
                offset + 1,
                at(target),
                offset + 5
            );
        }
        script += &format!("substr($o, $p, {width}) = $c;\n");
    }
    script + &format!("seek F, 0, 0; print F $o;\n' {path}\n")
}

fn powershell(profile: &Profile) -> String {
    let at = |anchor: &Anchor| format!("(At '{}') + {}", regex(anchor.signature), anchor.offset);
    let mut script = format!(
        "& {{
$f = \"{WINDOWS}\"
$b = [IO.File]::ReadAllBytes($f)
$s = [Text.Encoding]::GetEncoding(28591).GetString($b)
function At($re) {{
    $m = [regex]::Matches($s, \"(?s)(?=$re)\")
    if ($m.Count -ne 1) {{ throw \"signature not found\" }}
    $m[0].Index
}}
"
    );
    for patch in profile.patches {
        let (bytes, calls) = code(&patch.action);
        let hex: Vec<_> = bytes.iter().map(|byte| format!("0x{byte:02x}")).collect();
        script += &format!(
            "$p = {}\nif ($s.Substring($p, {}) -cnotmatch '(?s)\\A{}\\z') {{ throw \"unexpected bytes\" }}\n$c = [byte[]]({})\n",
            at(&patch.anchor),
            width(patch.expected),
            regex(patch.expected),
            hex.join(",")
        );
        for (offset, target) in calls {
            script += &format!(
                "[BitConverter]::GetBytes([int]({} - $p - {})).CopyTo($c, {})\n",
                at(target),
                offset + 5,
                offset + 1
            );
        }
        script += "$c.CopyTo($b, $p)\n";
    }
    script + "Copy-Item $f \"$f.bak\"\n[IO.File]::WriteAllBytes($f, $b)\n}\n"
}

fn hashes(profile: &Profile, command: &str) -> String {
    let mut text = format!(
        "check the result with `{command}`:\n\n| version | original | patched |\n| --- | --- | --- |\n"
    );
    for build in profile.builds {
        text += &format!(
            "| {} | `{}` | `{}` |\n",
            build.version, build.original_sha256, build.patched_sha256
        );
    }
    text
}

fn linux(profile: &Profile) -> String {
    format!(
        "close resolve, then back up the executable:\n\n```sh\nsudo cp -pn {LINUX} {LINUX}.bak\n```\n\n\
         patch:\n\n```sh\n{}```\n\n{}\nrestore:\n\n```sh\nsudo cp -p {LINUX}.bak {LINUX}\n```\n",
        perl(profile, LINUX),
        hashes(profile, &format!("sha256sum {LINUX}"))
    )
}

fn windows(profile: &Profile) -> String {
    format!(
        "close resolve, then run in powershell as administrator. the backup is saved as `Resolve.exe.bak`:\n\n\
         ```powershell\n{}```\n\n{}\nrestore:\n\n```powershell\nCopy-Item \"{WINDOWS}.bak\" \"{WINDOWS}\"\n```\n",
        powershell(profile),
        hashes(profile, &format!("Get-FileHash \"{WINDOWS}\""))
    )
}

fn macos(id: &str) -> String {
    let mut text = format!(
        "close resolve, then back up the executable:\n\n```sh\nsudo cp -pn {MACOS} {MACOS_BACKUP}\n```\n\n\
         patch each slice that `lipo -archs {MACOS}` lists.\n\n"
    );
    for profile in profiles::ALL.iter().filter(|profile| profile.id == id) {
        let architecture = match profile.architecture {
            Architecture::X86_64 => "x86_64",
            Architecture::Arm64 => "arm64",
        };
        text += &format!("{architecture}:\n\n```sh\n{}```\n\n", perl(profile, MACOS));
    }
    text + &format!(
        "sign:\n\n```sh\nsudo codesign --force --sign - --preserve-metadata=entitlements {MACOS}\n```\n\n\
         restore:\n\n```sh\nsudo cp -p {MACOS_BACKUP} {MACOS}\n```\n"
    )
}

fn main() {
    fs::create_dir_all("manual").unwrap();
    for (index, profile) in profiles::ALL.iter().enumerate() {
        let id = profile.id;
        if profiles::ALL[..index].iter().any(|other| other.id == id) {
            continue;
        }
        let steps = match profile.platform {
            Platform::Linux => linux(profile),
            Platform::Windows => windows(profile),
            Platform::Macos => macos(id),
        };
        let text = format!(
            "# {id}\n\ngenerated from `src/profiles/` by `cargo run --example manual`\n\n{steps}"
        );
        fs::write(format!("manual/{id}.md"), text).unwrap();
    }
}
