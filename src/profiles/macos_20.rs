use crate::{
    binary::{Architecture, Platform},
    profile::{Action, Anchor, Build, Patch, Profile, Rel8, Verify},
    version::{Version, VersionRange},
};

const VERSIONS: VersionRange = VersionRange {
    min: Version([20, 0, 0, 0]),
    max: Version([20, 2, u32::MAX, u32::MAX]),
};

pub const X86_64: Profile = Profile {
    id: "macos-20",
    platform: Platform::Macos,
    architecture: Architecture::X86_64,
    versions: VERSIONS,
    builds: &[Build {
        version: Version([20, 0, 0, 49]),
        original_sha256: "7cbd8e87e9c0b441a4fd34702d0abc32755a922d1f5a107db06fc347935c2ec2",
        patched_sha256: "0ef63233f6fa752587b55b744db02ef62e075624ca5a17f2613eecbe5a9f701a",
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
                signature: "E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01 E9 ?? ?? ?? ?? 48 8D 05",
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
    id: "macos-20",
    platform: Platform::Macos,
    architecture: Architecture::Arm64,
    versions: VERSIONS,
    builds: &[Build {
        version: Version([20, 0, 0, 49]),
        original_sha256: "8fc7d3f87f02cc4a02388e8a4bf99cbd3c3209cd0bd8dfdd76f4fa3b6cabb286",
        patched_sha256: "d614a2fa527ff28d94609cfe3db0d33e436d8509b4820613fd0e7b3d6520fe6a",
    }],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "FD 7B 05 A9 FD 43 01 91 ?? ?? ?? ?? 01 00 80 52 02 00 80 D2 ?? ?? ?? 94 ?? ?? ?? 34",
            offset: 8,
        },
        expected: "?? ?? ?? 97",
        action: Action::Arm64Branch {
            target: Anchor {
                signature: "?? ?? ?? 94 ?? ?? ?? 94 20 00 80 52 29 00 00 14",
                offset: 0,
            },
            link: false,
            verify: Some(24),
        },
    }],
};
