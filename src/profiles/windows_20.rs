use crate::{
    binary::Platform,
    profile::{Action, Anchor, Build, Patch, Profile, Rel32, Verify},
    version::{Version, VersionRange},
};

pub const PROFILE: Profile = Profile {
    id: "windows-20",
    platform: Platform::Windows,
    architecture: crate::binary::Architecture::X86_64,
    versions: VersionRange {
        min: Version([20, 0, 0, 0]),
        max: Version([20, u32::MAX, u32::MAX, u32::MAX]),
    },
    builds: &[
        Build {
            version: Version([20, 0, 0, 49]),
            original_sha256: "8b2dc96b4118c22df51c5aa9c07175c34b014e624259a8a4da591f452a360c20",
            patched_sha256: "9e92359dec5adce36e38cd768773880f424937e68a2c363180bb63882875faf9",
        },
        Build {
            version: Version([20, 3, 3, 10]),
            original_sha256: "9f800eb095d702adbb224c3e2b55f1ad0c55f1a116aaa7a2a936cd88c45a3abc",
            patched_sha256: "efdfa2c6cc9b5d744393ce6ab8be368896f2fb266800ab5c1e4470badc6bd5a5",
        },
    ],
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
