use crate::{
    binary::{Architecture, Platform},
    profile::{Action, Anchor, Build, Patch, Profile, Rel8, Verify},
    version::{Version, VersionRange},
};

const VERSIONS: VersionRange = VersionRange {
    min: Version([21, 0, 0, 0]),
    max: Version([21, 0, 3, u32::MAX]),
};

pub const X86_64: Profile = Profile {
    id: "macos-21.0",
    platform: Platform::Macos,
    architecture: Architecture::X86_64,
    versions: VERSIONS,
    builds: &[Build {
        version: Version([21, 0, 0, 48]),
        original_sha256: "012285456be34e4d1b6bc142703482a105c8e3e9e446ad7640fbf2d56982a623",
        patched_sha256: "903196e1421b19c0c0f5352ca7466d209b69a9ad15106de85ad2766158cf63b5",
    }],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "55 48 89 E5 53 48 83 EC 48 ?? ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? 84 C0 75 ?? E8 ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 74 ??",
            offset: 9,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01 48 83 C4 48 5B 5D C3",
                offset: 0,
            },
            verify: Some(Verify::Rel8(Rel8 {
                displacement_offset: 46,
                instruction_end: 47,
            })),
        },
    }],
};

pub const ARM64: Profile = Profile {
    id: "macos-21.0",
    platform: Platform::Macos,
    architecture: Architecture::Arm64,
    versions: VERSIONS,
    builds: &[Build {
        version: Version([21, 0, 0, 48]),
        original_sha256: "50d482305d511bfd5ed097c5014aa7ce6bd00d20730cdd57509699d97453bae0",
        patched_sha256: "b5647857d681b96f8bb94c5cc2322caeccc0c691447f9b7e387a1f9cff770e51",
    }],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "FF 83 01 D1 F4 4F 04 A9 FD 7B 05 A9 FD 43 01 91 ?? ?? ?? ?? ?? ?? ?? 97 C0 00 00 37 ?? ?? ?? 97 01 00 80 52 02 00 80 D2 ?? ?? ?? 94 ?? ?? ?? 34",
            offset: 16,
        },
        expected: "?? ?? ?? 97",
        action: Action::Arm64Branch {
            target: Anchor {
                signature: "?? ?? ?? 94 ?? ?? ?? 94 20 00 80 52 FD 7B 45 A9 F4 4F 44 A9 FF 83 01 91 C0 03 5F D6",
                offset: 0,
            },
            link: false,
            verify: Some(44),
        },
    }],
};
