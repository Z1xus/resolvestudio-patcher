use crate::{
    profile::{Action, Anchor, Build, Patch, Profile, Rel32, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    platform: crate::binary::Platform::Linux,
    architecture: crate::binary::Architecture::X86_64,
    id: "linux-21.2",
    versions: VersionRange {
        min: Version([21, 1, 1, 0]),
        max: Version([21, 2, u32::MAX, u32::MAX]),
    },
    builds: &[
        Build {
            version: Version([21, 1, 1, 10]),
            original_sha256: "2462f80bff8d545516fbc85a7fb470811b4f1ae75a0dfff8026bf6db7c3483e5",
            patched_sha256: "3fd49055041e7225da16220dca914c509061ff153f2170203ee45d20d5209ae7",
        },
        Build {
            version: Version([21, 2, 0, 6]),
            original_sha256: "e38b3f27b8b15c1cd844753e9126741dd5dfdb2c56dade4cc4d5b573963cdc56",
            patched_sha256: "fa4d58d03c2e66cf3e599691bedc04a89fd2ccead248bd2ef13ff402307671c6",
        },
    ],
    patches: &[
        Patch {
            name: "startup",
            anchor: Anchor {
                signature: "?? ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? 84 C0 75 19 E8 ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 0F 84 ?? ?? ?? ??",
                offset: 0,
            },
            expected: "E8 ?? ?? ?? ??",
            action: Action::Jump {
                target: Anchor {
                    signature: "BF 01 00 00 00 E8 ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01",
                    offset: 0,
                },
                verify: Some(Verify::Rel32(Rel32 {
                    displacement_offset: 38,
                    instruction_end: 42,
                })),
            },
        },
        Patch {
            name: "write-guard",
            anchor: Anchor {
                signature: "48 8B 7C 24 20 BE 02 00 00 00 BA 08 00 00 00 E8 ?? ?? ?? ?? 41 09 EE ?? ?? 48 89 DF E8",
                offset: 23,
            },
            expected: "75 60",
            action: Action::Bytes(&[0xEB, 0x60]),
        },
        Patch {
            name: "render-guard",
            anchor: Anchor {
                signature: "48 B9 CC CC CC CC CC CC CC 0C 48 39 C8 0F 87 ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? 48 3D",
                offset: 34,
            },
            expected: "C1 27 09 00",
            action: Action::AllBytes(&[0xFF, 0xFF, 0xFF, 0x7F]),
        },
    ],
};
