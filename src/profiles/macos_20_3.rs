use crate::{
    binary::{Architecture, Platform},
    profile::{Action, Anchor, Build, Patch, Profile, Rel8, Verify},
    version::{Version, VersionRange},
};

const VERSIONS: VersionRange = VersionRange {
    min: Version([20, 3, 0, 0]),
    max: Version([20, u32::MAX, u32::MAX, u32::MAX]),
};

pub const X86_64: Profile = Profile {
    id: "macos-20.3",
    platform: Platform::Macos,
    architecture: Architecture::X86_64,
    versions: VERSIONS,
    builds: &[Build {
        version: Version([20, 3, 3, 10]),
        original_sha256: "527fed290b7f457d03421f2538fc2f0305ce5f4e51fd0d13e3248f5e9ddae9d6",
        patched_sha256: "0d94ee69d76083382e2c4d47c3dfe0433ca1e7e0587ef2f3a4d78352775ab625",
    }],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "55 48 89 E5 53 48 83 EC 48 ?? ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 74 ??",
            offset: 9,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01 48 83 C4 48 5B 5D C3",
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
    id: "macos-20.3",
    platform: Platform::Macos,
    architecture: Architecture::Arm64,
    versions: VERSIONS,
    builds: &[Build {
        version: Version([20, 3, 3, 10]),
        original_sha256: "e62ee24ea8292a4ad0e102e2a3fe7f0d3a1acc0f69ef5a157ffce4bcc6bc046c",
        patched_sha256: "6ff45e1ff541b0ef6f0f252fdd15f13757d582699ba27146761f7af54c789c2b",
    }],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "FF 83 01 D1 F4 4F 04 A9 FD 7B 05 A9 FD 43 01 91 ?? ?? ?? ?? 01 00 80 52 02 00 80 D2 ?? ?? ?? 94 ?? ?? ?? 34",
            offset: 16,
        },
        expected: "?? ?? ?? 97",
        action: Action::Arm64Branch {
            target: Anchor {
                signature: "?? ?? ?? 94 ?? ?? ?? 94 20 00 80 52 FD 7B 45 A9 F4 4F 44 A9 FF 83 01 91 C0 03 5F D6",
                offset: 0,
            },
            link: false,
            verify: Some(32),
        },
    }],
};
