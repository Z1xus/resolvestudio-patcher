use crate::{
    binary::Platform,
    profile::{Action, Anchor, Patch, Profile, Rel32, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    id: "windows-19",
    platform: Platform::Windows,
    architecture: crate::binary::Architecture::X86_64,
    versions: VersionRange {
        min: Version([19, 0, 0, 0]),
        max: Version([19, 0, u32::MAX, u32::MAX]),
    },
    builds: &[],
    patches: &[
        Patch {
            name: "startup",
            anchor: Anchor {
                signature: "48 89 5C 24 10 57 48 81 EC 80 00 00 00 33 FF ?? ?? ?? ?? ?? 45 33 C0 33 D2 48 8B C8 E8 ?? ?? ?? ??",
                offset: 15,
            },
            expected: "E8 ?? ?? ?? ??",
            action: Action::Jump {
                target: Anchor {
                    signature: "E8 ?? ?? ?? ?? 48 8B C8 E8 ?? ?? ?? ?? B0 01 48 8B 9C 24 98 00 00 00 48 81 C4 80 00 00 00 5F C3",
                    offset: 0,
                },
                verify: Some(Verify::Rel32(Rel32 {
                    displacement_offset: 37,
                    instruction_end: 41,
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
