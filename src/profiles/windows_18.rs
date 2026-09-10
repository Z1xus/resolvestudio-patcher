use crate::{
    binary::Platform,
    profile::{Action, Anchor, Build, Patch, Profile, Rel8, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    id: "windows-18",
    platform: Platform::Windows,
    architecture: crate::binary::Architecture::X86_64,
    versions: VersionRange {
        min: Version([18, 0, 0, 0]),
        max: Version([18, 6, 2, u32::MAX]),
    },
    builds: &[Build {
        version: Version([18, 0, 0, 36]),
        original_sha256: "a91e06a1c2f5e7d4d7ce27301c8350f9957dfe985539cec613dd54d9c386bd7d",
        patched_sha256: "dab6336880fcb771b085a3d1aa1eb9cc731d2624f25f2032345c2fabea7ddea9",
    }],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            // jump only after the local QString objects have been initialized.
            signature: "48 8D 4D 07 FF 15 ?? ?? ?? ?? 48 8D 4D FF FF 15 ?? ?? ?? ?? 90 80 7D 0F 00 0F 85 ?? ?? ?? ?? ?? ?? ?? ?? ?? 48 8B C8 45 33 C0 33 D2 E8 ?? ?? ?? ?? 85 C0 74 ??",
            offset: 31,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "E8 ?? ?? ?? ?? 48 8B C8 E8 ?? ?? ?? ?? B3 01 48 8D 4D 1F FF 15 ?? ?? ?? ??",
                offset: 0,
            },
            verify: Some(Verify::Rel8(Rel8 {
                displacement_offset: 52,
                instruction_end: 53,
            })),
        },
    }],
};
