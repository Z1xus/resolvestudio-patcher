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
        patched_sha256: "5de7d901e0fd14ed5dc00e1b7f82512d5ee542ac081927dce054bb989a888522",
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
                signature: "48 89 DF E8 ?? ?? ?? ?? 66 66 66 66 66 66 2E 0F 1F 84 00 00 00 00 00 55 48 89 E5 ?? ?? ?? ?? 5D C3 66 0F 1F 44 00 00 55 48 89 E5 0F B6 47 0A 5D C3",
                offset: 27,
            },
            expected: "0F B6 47 09",
            action: Action::Bytes(&[0x31, 0xC0, 0xB0, 0x01]),
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
        patched_sha256: "60393e5f008438a62fabe4cade2c39c7ecfefd4b5459f399c6d7608234c676ff",
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
            name: "activation",
            anchor: Anchor {
                signature: "?? ?? ?? ?? C0 03 5F D6 00 28 40 39 C0 03 5F D6 08 20 40 39 1F 05 00 71 A1 03 00 54",
                offset: 0,
            },
            expected: "00 24 40 39",
            action: Action::Bytes(&[0x20, 0x00, 0x80, 0x52]),
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
