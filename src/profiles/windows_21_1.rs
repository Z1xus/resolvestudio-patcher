use crate::{
    binary::Platform,
    profile::{Action, Anchor, Build, Call, Patch, Profile},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    id: "windows-21.1",
    platform: Platform::Windows,
    architecture: crate::binary::Architecture::X86_64,
    versions: VersionRange {
        min: Version([21, 0, 4, 0]),
        max: Version([21, 2, u32::MAX, u32::MAX]),
    },
    builds: &[
        Build {
            version: Version([21, 0, 4, 5]),
            original_sha256: "5dd37102210dc1907dd196240c2cf47186a1cb2686cab0357754927b7e688d39",
            patched_sha256: "ba4ec5e96adb7089d0611c1fad9e6c39198c4f81fb97487f310d836481077402",
        },
        Build {
            version: Version([21, 1, 0, 14]),
            original_sha256: "0a8f20c70851e629c1753ed537801d1df0b5162ad98a5ab399e4abc7b77df236",
            patched_sha256: "b9676fae37c87f3df8ccf67dd65e2e82c8e22ebac10d4b19adc7981e8ba72c11",
        },
        Build {
            version: Version([21, 2, 0, 6]),
            original_sha256: "58ea8835bc2f3a8b5266978178f0a3833474ad3095e4e9c1b319d9a7e50ae150",
            patched_sha256: "40dd70d48c07eb5bd378b8e3a99bc2683589f985837881da38a4e9a6f2cd7033",
        },
    ],
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
    }],
};
