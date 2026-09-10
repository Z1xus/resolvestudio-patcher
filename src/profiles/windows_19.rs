use crate::{
    binary::Platform,
    profile::{Action, Anchor, Build, Patch, Profile, Rel32, Verify},
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
    builds: &[Build {
        version: Version([19, 0, 0, 69]),
        original_sha256: "36ba6b56c540ba74cce5c2525519f95f9c8864ff69b835e920aada4ddd9c3996",
        patched_sha256: "ac03226900011a171f5fe6dc648ac03113d769c9f35ccad56b310c47152eceef",
    }],
    patches: &[Patch {
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
    }],
};
