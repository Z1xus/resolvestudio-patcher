use crate::{
    profile::{Action, Anchor, Build, Patch, Profile, Rel32},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    platform: crate::binary::Platform::Linux,
    architecture: crate::binary::Architecture::X86_64,
    id: "linux-21.1",
    versions: VersionRange {
        min: Version([21, 1, 0, 0]),
        max: Version([21, 1, u32::MAX, u32::MAX]),
    },
    builds: &[Build {
        version: Version([21, 1, 0, 14]),
        original_sha256: "23d1bedf6f87fc26979cdaf1b4b5c3beef3bbcbefb34207a750450adef29fea2",
        patched_sha256: "3861df318072d83a187dd8a6aaf9136c055137c611fa28e1915fa61ff1e3e8a3",
    }],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "53 48 83 EC 40 ?? ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? 84 C0 75 19 E8 ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 0F 84 ?? ?? ?? ??",
            offset: 5,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "BF 01 00 00 00 E8 ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01 48 83 C4 40 5B C3",
                offset: 0,
            },
            verify: Some(Rel32 {
                displacement_offset: 43,
                instruction_end: 47,
            }),
        },
    }],
};
