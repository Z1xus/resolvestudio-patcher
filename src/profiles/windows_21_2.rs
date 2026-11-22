use crate::{
    binary::Platform,
    profile::{Action, Anchor, Build, Call, Patch, Profile},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    id: "windows-21.2",
    platform: Platform::Windows,
    architecture: crate::binary::Architecture::X86_64,
    versions: VersionRange {
        min: Version([21, 1, 1, 0]),
        max: Version([21, 2, u32::MAX, u32::MAX]),
    },
    builds: &[
        Build {
            version: Version([21, 1, 1, 10]),
            original_sha256: "1a109e719f2d5e20167ecd9e0f1a37407cf62702fcea366e5e7f6dbbdb826f83",
            patched_sha256: "7f202c70454f9b5e8bd202632357e98cf61f60b07cf275872077740ef97579e2",
        },
        Build {
            version: Version([21, 2, 0, 6]),
            original_sha256: "58ea8835bc2f3a8b5266978178f0a3833474ad3095e4e9c1b319d9a7e50ae150",
            patched_sha256: "52bafadce253118960bd7d32f73c4a6912cb58886fe1168bd5e305548717f694",
        },
    ],
    patches: &[
        Patch {
            name: "startup",
            anchor: Anchor {
                signature: "40 53 48 81 EC 80 00 00 00 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 4C 8D 05 ?? ?? ?? ?? 48 8D 15 ?? ?? ?? ?? 48 8D 8C 24 90 00 00 00 FF 15 ?? ?? ?? ?? 90",
                offset: 9,
            },
            expected: "E8 ?? ?? ?? ?? 84 C0 74 0B B0 01 48 81 C4 80 00 00 00 5B C3 C7 44 24 20 FF FF FF FF 45 33 C9",
            action: Action::Code {
                // initialize features and return, preserving the prologue and unwind data.
                bytes: &[
                    0xe8, 0, 0, 0, 0, 0x48, 0x8b, 0xc8, 0xe8, 0, 0, 0, 0, 0xb0, 0x01, 0x48, 0x81,
                    0xc4, 0x80, 0, 0, 0, 0x5b, 0xc3, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
                ],
                calls: &[
                    Call {
                        follow: true,
                        offset: 0,
                        target: Anchor {
                            signature: "E8 ?? ?? ?? ?? 48 8B C8 E8 ?? ?? ?? ?? B3 01 48 8D 4C 24 ?? FF 15 ?? ?? ?? ??",
                            offset: 0,
                        },
                    },
                    Call {
                        follow: true,
                        offset: 8,
                        target: Anchor {
                            signature: "E8 ?? ?? ?? ?? 48 8B C8 E8 ?? ?? ?? ?? B3 01 48 8D 4C 24 ?? FF 15 ?? ?? ?? ??",
                            offset: 8,
                        },
                    },
                ],
            },
        },
        Patch {
            name: "write-guard",
            anchor: Anchor {
                signature: "85 FF ?? ?? 85 DB 75 ?? 49 8B CE E8 ?? ?? ?? ?? 48 8B F0 49 8B CE E8 ?? ?? ?? ?? 8B D8 E8",
                offset: 2,
            },
            expected: "75 5B",
            action: Action::Bytes(&[0xEB, 0x5B]),
        },
        Patch {
            name: "render-guard",
            anchor: Anchor {
                signature: "B8 67 66 66 66 F7 E9 C1 FA 03 8B C2 C1 E8 1F 03 D0 8D 04 92 C1 E0 02 3B C8 0F 85 ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B C8 E8 ?? ?? ?? ?? 3D",
                offset: 45,
            },
            expected: "C0 27 09 00",
            action: Action::AllBytes(&[0xFF, 0xFF, 0xFF, 0x7F]),
        },
    ],
};
