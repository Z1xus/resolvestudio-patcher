use crate::{
    profile::{Action, Anchor, Build, Patch, Profile, Rel8, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    platform: crate::binary::Platform::Linux,
    architecture: crate::binary::Architecture::X86_64,
    id: "linux-20",
    versions: VersionRange {
        min: Version([20, 0, 0, 0]),
        max: Version([20, u32::MAX, u32::MAX, u32::MAX]),
    },
    builds: &[
        Build {
            version: Version([20, 0, 0, 49]),
            original_sha256: "f4e259d4bc9a8db672745c73cb94e63fb31ffff4a7688ff8dd5b073ef3346c8b",
            patched_sha256: "6e1d522b36c3141d84efb90c51373462908cc1c8ada5701c36c5da0806766cfe",
        },
        Build {
            version: Version([20, 3, 3, 10]),
            original_sha256: "e5ca88cce85a6aa1d749bc24ed59c22fbd06918fb0a40c926abc9d99cf5eaf48",
            patched_sha256: "d76f99cbab46272c2833cc6a72f8cd45435ca0d3b0efa1db47d8fddf5093a95d",
        },
    ],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "53 48 83 EC 40 ?? ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 74 ??",
            offset: 5,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? 84 C0 74 ?? E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01",
                offset: 17,
            },
            verify: Some(Verify::Rel8(Rel8 {
                displacement_offset: 25,
                instruction_end: 26,
            })),
        },
    }],
};
