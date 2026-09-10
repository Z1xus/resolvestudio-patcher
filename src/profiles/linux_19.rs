use crate::{
    profile::{Action, Anchor, Build, Patch, Profile, Rel8, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    platform: crate::binary::Platform::Linux,
    architecture: crate::binary::Architecture::X86_64,
    id: "linux-19",
    versions: VersionRange {
        min: Version([19, 0, 0, 0]),
        max: Version([19, u32::MAX, u32::MAX, u32::MAX]),
    },
    builds: &[
        Build {
            version: Version([19, 0, 0, 69]),
            original_sha256: "137766a536d50b9fec37ec0e53115445db85f285f09d9e187e94858a8ccd7594",
            patched_sha256: "2b1c8b943444fd9a732ec077df20967c6ce1fb68878c85eb1a800607320f1257",
        },
        Build {
            version: Version([19, 1, 4, 11]),
            original_sha256: "bbd78d9c768c7d2ff284134bb7e900fe35c889a435a09ab69d7ccb46aa6d44ae",
            patched_sha256: "b1d028e74985262b9a48b01c1a2af9a1e8319dc4e952af3e2e36378624af3aed",
        },
    ],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "53 48 83 EC 40 ?? ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 74 ?? E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? 84 C0 74 ??",
            offset: 5,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01 E9 ?? ?? ?? ?? 48 C7 C0",
                offset: 0,
            },
            verify: Some(Verify::Rel8(Rel8 {
                displacement_offset: 25,
                instruction_end: 26,
            })),
        },
    }],
};
