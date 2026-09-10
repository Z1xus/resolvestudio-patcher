use crate::{
    binary::Platform,
    profile::{Action, Anchor, Build, Patch, Profile, Rel32, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    id: "windows-19.1",
    platform: Platform::Windows,
    architecture: crate::binary::Architecture::X86_64,
    versions: VersionRange {
        min: Version([19, 1, 0, 0]),
        max: Version([19, 1, u32::MAX, u32::MAX]),
    },
    builds: &[Build {
        version: Version([19, 1, 4, 11]),
        original_sha256: "50bbfe69234a81302f123a63e84b0ff7f62327ae62b6909c97261590a003366b",
        patched_sha256: "93e2499a61a9c2e972009d535fe85db0ee769094611693e0e378924fefd75ced",
    }],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            // use the helper's prologue so the destination restores the same stack frame.
            signature: "48 89 5C 24 10 57 48 81 EC 80 00 00 00 33 DB ?? ?? ?? ?? ?? 45 33 C0 33 D2 48 8B C8 E8 ?? ?? ?? ?? 85 C0 0F 84 ?? ?? ?? ??",
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
