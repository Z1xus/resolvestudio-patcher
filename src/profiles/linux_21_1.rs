use crate::{
    profile::{Action, Anchor, Build, Patch, Profile, Rel32, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    platform: crate::binary::Platform::Linux,
    architecture: crate::binary::Architecture::X86_64,
    id: "linux-21.1",
    versions: VersionRange {
        min: Version([21, 0, 4, 0]),
        max: Version([21, 1, 0, u32::MAX]),
    },
    builds: &[
        Build {
            version: Version([21, 0, 4, 5]),
            original_sha256: "1092888fe6fc8e12339a9ea0fc38985089d486a8f9b97bc8ec922649e866fbbd",
            patched_sha256: "5aa7e1459c99ae6e9ed2caf67d418024f862ab0b83083e8d48c75b631563e258",
        },
        Build {
            version: Version([21, 1, 0, 17]),
            original_sha256: "99d4e0a46d63cbfe71ce4b30f23c5654ffbdd17d782d9368986cecd159e415d0",
            patched_sha256: "d681880c11b8aac4c50791355bb3dd43557a4eb80c48d782f3b2f99ccb167b88",
        },
        Build {
            version: Version([21, 2, 0, 6]),
            original_sha256: "e38b3f27b8b15c1cd844753e9126741dd5dfdb2c56dade4cc4d5b573963cdc56",
            patched_sha256: "c49b889ef90ecb38bbeba846d41191f9fc685046683f59b8096fbde07e82a278",
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
