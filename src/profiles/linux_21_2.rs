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
    builds: &[Build {
        version: Version([21, 2, 0, 6]),
        original_sha256: "e38b3f27b8b15c1cd844753e9126741dd5dfdb2c56dade4cc4d5b573963cdc56",
        patched_sha256: "bfd76d22d3659b9808ecac25d2d9125081f5cb70cb84bea6ea09124ae2d7d32b",
    }],
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
            name: "activation",
            anchor: Anchor {
                signature: "48 89 C3 48 8D 7C 24 18 E8 ?? ?? ?? ?? 48 8D 7C 24 10 E8 ?? ?? ?? ?? 48 8D 7C 24 08 E8 ?? ?? ?? ?? 48 89 DF E8 ?? ?? ?? ?? 0F 1F 80 00 00 00 00 ?? ?? ?? ?? ?? 66 66 2E 0F 1F 84 00 00 00 00 00",
                offset: 48,
            },
            expected: "0F B6 47 09 C3",
            action: Action::Bytes(&[0xB0, 0x01, 0xC3, 0x90, 0x90]),
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
