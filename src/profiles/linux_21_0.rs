use crate::{
    profile::{Action, Anchor, Build, Patch, Profile, Rel8, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    platform: crate::binary::Platform::Linux,
    architecture: crate::binary::Architecture::X86_64,
    id: "linux-21.0",
    versions: VersionRange {
        min: Version([21, 0, 0, 0]),
        max: Version([21, 0, 3, u32::MAX]),
    },
    builds: &[Build {
        version: Version([21, 0, 0, 48]),
        original_sha256: "83201c3fefb5b76275abda255621db27196d0e9b22eee0e1fd491862642ea389",
        patched_sha256: "b59c39785fab2bde57d5aff37457988542e189150aad1672366aa6f979c16b54",
    }],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "53 48 83 EC 40 ?? ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? 84 C0 75 ?? E8 ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 74 ??",
            offset: 5,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01 48 83 C4 40 5B C3",
                offset: 0,
            },
            verify: Some(Verify::Rel8(Rel8 {
                displacement_offset: 42,
                instruction_end: 43,
            })),
        },
    }],
};
