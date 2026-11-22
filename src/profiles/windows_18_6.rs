use crate::{
    binary::Platform,
    profile::{Action, Anchor, Patch, Profile, Rel32, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    id: "windows-18.6",
    platform: Platform::Windows,
    architecture: crate::binary::Architecture::X86_64,
    versions: VersionRange {
        min: Version([18, 6, 3, 0]),
        max: Version([18, 6, u32::MAX, u32::MAX]),
    },
    builds: &[],
    patches: &[
        Patch {
            name: "startup",
            anchor: Anchor {
                // keep the local result and its QString members initialized before the jump.
                signature: "48 8D 4C 24 30 E8 ?? ?? ?? ?? 90 80 7C 24 30 00 0F 85 ?? ?? ?? ?? ?? ?? ?? ?? ?? 45 33 C0 33 D2 48 8B C8 E8 ?? ?? ?? ?? 85 C0 0F 84 ?? ?? ?? ??",
                offset: 22,
            },
            expected: "E8 ?? ?? ?? ??",
            action: Action::Jump {
                target: Anchor {
                    signature: "E8 ?? ?? ?? ?? 48 8B C8 E8 ?? ?? ?? ?? B3 01 48 8D 4C 24 ?? FF 15 ?? ?? ?? ??",
                    offset: 0,
                },
                verify: Some(Verify::Rel32(Rel32 {
                    displacement_offset: 44,
                    instruction_end: 48,
                })),
            },
        },
        Patch {
            name: "write-guard",
            anchor: Anchor {
                signature: "85 FF ?? ?? 85 DB 75 ?? 49 8B CE E8 ?? ?? ?? ?? 48 8B F0",
                offset: 2,
            },
            expected: "75",
            action: Action::Bytes(&[0xEB]),
        },
    ],
};
