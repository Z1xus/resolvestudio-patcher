use crate::{
    binary::Platform,
    profile::{Action, Anchor, Build, Call, Patch, Profile},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    id: "windows-21.1",
    platform: Platform::Windows,
    versions: VersionRange {
        min: Version([21, 1, 0, 0]),
        max: Version([21, 1, u32::MAX, u32::MAX]),
    },
    builds: &[Build {
        version: Version([21, 1, 0, 14]),
        original_sha256: "0a8f20c70851e629c1753ed537801d1df0b5162ad98a5ab399e4abc7b77df236",
        patched_sha256: "b9676fae37c87f3df8ccf67dd65e2e82c8e22ebac10d4b19adc7981e8ba72c11",
    }],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "40 53 48 81 EC 80 00 00 00 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 4C 8D 05 ?? ?? ?? ?? 48 8D 15 ?? ?? ?? ?? 48 8D 8C 24 90 00 00 00 FF 15 ?? ?? ?? ?? 90",
            offset: 9,
        },
        expected: "E8 ?? ?? ?? ?? 84 C0 74 0B B0 01 48 81 C4 80 00 00 00 5B C3 C7 44 24 20 FF FF FF FF 45 33 C9",
        action: Action::Code {
            // initialize features and return, preserving the prologue and unwind data.
            bytes: &[
                0xe8, 0, 0, 0, 0, 0x48, 0x8b, 0xc8, 0xe8, 0, 0, 0, 0, 0xb0, 0x01, 0x48, 0x81, 0xc4,
                0x80, 0, 0, 0, 0x5b, 0xc3, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
            ],
            calls: &[
                Call {
                    offset: 0,
                    target: Anchor {
                        signature: "48 89 5C 24 10 57 48 83 EC 30 E8 ?? ?? ?? ?? 48 8B D8 48 83 38 00 75 ?? 48 8D 78 08 48 89 7C 24 20 C6 44 24 28 00 48 8B CF E8 ?? ?? ?? ?? 85 C0 75 ?? 81 7F 4C FF FF FF 7F 74 ?? C6 44 24 28 01 48 83 3B 00 75 ?? B9 B0 08 00 00",
                        offset: 0,
                    },
                },
                Call {
                    offset: 8,
                    target: Anchor {
                        signature: "48 8D 41 30 B9 00 04 00 00 0F 1F 80 00 00 00 00 C6 80 00 04 00 00 01 C6 00 01 48 8D 40 01 48 83 E9 01 75 EC C3",
                        offset: 0,
                    },
                },
            ],
        },
    }],
};
