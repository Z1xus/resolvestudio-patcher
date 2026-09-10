use crate::{
    profile::{Action, Anchor, Build, Patch, Profile, Rel8, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    platform: crate::binary::Platform::Linux,
    architecture: crate::binary::Architecture::X86_64,
    id: "linux-18",
    versions: VersionRange {
        min: Version([18, 0, 0, 0]),
        max: Version([18, u32::MAX, u32::MAX, u32::MAX]),
    },
    builds: &[
        Build {
            version: Version([18, 0, 0, 36]),
            original_sha256: "4a1cd88ceafdf91bbcdb3157998ab7020fba24dbd37f453dc789b40aca3fb884",
            patched_sha256: "18db894b2aaf43edfc25041290828f0503e009931fac9493ff2f43a9e21f612b",
        },
        Build {
            version: Version([18, 6, 6, 7]),
            original_sha256: "6e4b8e707efe8b5e16d40dbbc22323a4be50e13e63024bac5a9675348b9eb93b",
            patched_sha256: "c3323be485812ba481a6303bb3d7f721b42d6b67027b6bac8c27279e9fa24fce",
        },
    ],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "53 48 83 EC 30 ?? ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 74 ?? E8 ?? ?? ?? ?? 48 8D 5C 24 08",
            offset: 5,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01 EB 0C 48 8D 7C 24 08",
                offset: 0,
            },
            verify: Some(Verify::Rel8(Rel8 {
                displacement_offset: 25,
                instruction_end: 26,
            })),
        },
    }],
};
