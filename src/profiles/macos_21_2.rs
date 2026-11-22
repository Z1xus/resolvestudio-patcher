use crate::{
    binary::{Architecture, Platform},
    profile::{Action, Anchor, Build, Patch, Profile, Rel32, Verify},
    version::{Version, VersionRange},
};

const VERSIONS: VersionRange = VersionRange {
    min: Version([21, 1, 1, 0]),
    max: Version([21, 2, u32::MAX, u32::MAX]),
};

pub const X86_64: Profile = Profile {
    id: "macos-21.2",
    platform: Platform::Macos,
    architecture: Architecture::X86_64,
    versions: VERSIONS,
    builds: &[Build {
        version: Version([21, 2, 0, 6]),
        original_sha256: "0dce18d978a877d4c40aa894d54f9e5d6d8db78950ab60c9b27b41dfa8859689",
        patched_sha256: "6747751ec1966cd95e454593a785733bf5f0f1ebf1628262d1ae71aa62eaceb7",
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
            name: "write-guard",
            anchor: Anchor {
                signature: "48 D1 C9 48 39 C1 ?? ?? ?? ?? ?? ?? 48 8D 3D",
                offset: 6,
            },
            expected: "0F 87 20 03 00 00",
            action: Action::Bytes(&[0xE9, 0x21, 0x03, 0x00, 0x00, 0x90]),
        },
        Patch {
            name: "render-guard",
            anchor: Anchor {
                signature: "E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? 48 3D",
                offset: 15,
            },
            expected: "C1 27 09 00",
            action: Action::AllBytes(&[0xFF, 0xFF, 0xFF, 0x7F]),
        },
    ],
};

pub const ARM64: Profile = Profile {
    id: "macos-21.2",
    platform: Platform::Macos,
    architecture: Architecture::Arm64,
    versions: VERSIONS,
    builds: &[Build {
        version: Version([21, 2, 0, 6]),
        original_sha256: "95ad56280c0a5c2498c79e99e7d287713e014a663a90a3e96dd353967d1999c3",
        patched_sha256: "1764c5059060dc2784a8cf4391e4a4e1ff0e569560df8eb3777ea58705a38490",
    }],
    patches: &[
        Patch {
            name: "startup",
            anchor: Anchor {
                signature: "?? ?? ?? ?? ?? ?? ?? 97 C0 00 00 37 ?? ?? ?? 97 01 00 80 52 02 00 80 D2 ?? ?? ?? 94 ?? ?? ?? 34",
                offset: 0,
            },
            expected: "?? ?? ?? 97",
            action: Action::Arm64Branch {
                target: Anchor {
                    signature: "20 00 80 52 ?? ?? ?? 94 ?? ?? ?? 94 ?? ?? ?? 94 20 00 80 52",
                    offset: 0,
                },
                link: false,
                verify: Some(28),
            },
        },
        Patch {
            name: "write-guard",
            anchor: Anchor {
                signature: "E8 E7 02 B2 A8 99 99 F2 E9 E7 01 B2 29 ED 7D 92 08 24 08 9B 08 05 C8 93 1F 01 09 EB ?? ?? ?? ??",
                offset: 28,
            },
            expected: "A8 17 00 54",
            action: Action::Bytes(&[0xBD, 0x00, 0x00, 0x14]),
        },
        Patch {
            name: "render-guard",
            anchor: Anchor {
                signature: "?? ?? ?? 94 ?? ?? ?? 94 28 F8 84 52 ?? ?? ?? ?? 1F 00 08 EB",
                offset: 12,
            },
            expected: "28 01 A0 72",
            action: Action::AllBytes(&[0xE8, 0xFF, 0xAF, 0x72]),
        },
    ],
};
