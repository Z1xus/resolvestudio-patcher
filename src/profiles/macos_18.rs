use crate::{
    binary::{Architecture, Platform},
    profile::{Action, Anchor, Build, Patch, Profile, Rel8, Verify},
    version::{Version, VersionRange},
};

const VERSIONS: VersionRange = VersionRange {
    min: Version([18, 0, 0, 0]),
    max: Version([18, u32::MAX, u32::MAX, u32::MAX]),
};

pub const X86_64: Profile = Profile {
    id: "macos-18",
    platform: Platform::Macos,
    architecture: Architecture::X86_64,
    versions: VERSIONS,
    builds: &[
        Build {
            version: Version([18, 0, 0, 36]),
            original_sha256: "314c43fd251faa7a7564ff256a21497b326d7455186eea774d2439ea7465f93e",
            patched_sha256: "2875a873fb6e7cc8702bcb4e55df787aa0623faccb28adb72a75606c7ee82688",
        },
        Build {
            version: Version([18, 6, 6, 7]),
            original_sha256: "1321f5095cda2ab0c9d3b4a7a78f13a93e0d8bbd6cf4bf159d54faa5c5124b2f",
            patched_sha256: "d8543bbc99dc734f7b99a1ac2ad7dc255d39f239cbe6b313c0d7532a28c90307",
        },
    ],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "55 48 89 E5 53 48 83 EC 38 ?? ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 74 ??",
            offset: 9,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01 EB 0B",
                offset: 0,
            },
            verify: Some(Verify::Rel8(Rel8 {
                displacement_offset: 29,
                instruction_end: 30,
            })),
        },
    }],
};

pub const ARM64: Profile = Profile {
    id: "macos-18",
    platform: Platform::Macos,
    architecture: Architecture::Arm64,
    versions: VERSIONS,
    builds: &[
        Build {
            version: Version([18, 0, 0, 36]),
            original_sha256: "d8efadcf025c32cb38445609976e29af7fae1065a5b68b5ccc2fdd088c328777",
            patched_sha256: "41853b90aa545f6e81ab01d73e527a715b5e46008b45de33ca7e75ea09966286",
        },
        Build {
            version: Version([18, 6, 6, 7]),
            original_sha256: "5f6dceb078bb423470bb86d102e81a4bcc0818b60374a39814e8520929b086d4",
            patched_sha256: "d36856d04aad7559ddd45830b3c0d8b4ceeb483af833b638914fbd46507a5cf6",
        },
    ],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "FD 7B 04 A9 FD 03 01 91 ?? ?? ?? ?? 01 00 80 52 02 00 80 D2 ?? ?? ?? 94 ?? ?? ?? 34",
            offset: 8,
        },
        expected: "?? ?? ?? 94",
        action: Action::Arm64Branch {
            target: Anchor {
                signature: "?? ?? ?? 94 ?? ?? ?? 94 20 00 80 52 04 00 00 14",
                offset: 0,
            },
            link: false,
            verify: Some(24),
        },
    }],
};
