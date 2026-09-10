use crate::{
    binary::{Architecture, Platform},
    profile::{Action, Anchor, Build, Patch, Profile, Rel32, Verify},
    version::{Version, VersionRange},
};

const VERSIONS: VersionRange = VersionRange {
    min: Version([21, 0, 4, 0]),
    max: Version([21, 1, u32::MAX, u32::MAX]),
};

pub const X86_64: Profile = Profile {
    id: "macos-21.1",
    platform: Platform::Macos,
    architecture: Architecture::X86_64,
    versions: VERSIONS,
    builds: &[
        Build {
            version: Version([21, 0, 4, 5]),
            original_sha256: "503e96cb8495e256264f9f4b55d61784bd414a5cc57b8af7f1296b63c3b6bf25",
            patched_sha256: "5d815576fa117f997e0c1aa053c1e5c955e14623e1afe5808ae5d51e4a689612",
        },
        Build {
            version: Version([21, 1, 0, 14]),
            original_sha256: "a8184b4121678b234fd09104a656eae76d0b991de2233c030d3661d58cc14a6b",
            patched_sha256: "788d787c1b0942d83ec231b11cbeaabc338c640bd5a720d28a1eccf8a7b3963e",
        },
    ],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "55 48 89 E5 53 48 83 EC 48 ?? ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? 84 C0 75 19 E8 ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 0F 84 ?? ?? ?? ??",
            offset: 9,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "BF 01 00 00 00 E8 ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01 48 83 C4 48 5B 5D C3",
                offset: 0,
            },
            verify: Some(Verify::Rel32(Rel32 {
                displacement_offset: 47,
                instruction_end: 51,
            })),
        },
    }],
};

pub const ARM64: Profile = Profile {
    id: "macos-21.1",
    platform: Platform::Macos,
    architecture: Architecture::Arm64,
    versions: VERSIONS,
    builds: &[
        Build {
            version: Version([21, 0, 4, 5]),
            original_sha256: "62aefebdd43903965b085e1c0d391f9641c08fbc9cd68eacbe3df59c693137d6",
            patched_sha256: "895fb8d43fc601113b48a671c33c664b6b99b0c4fad2722d6350f610c23f5768",
        },
        Build {
            version: Version([21, 1, 0, 14]),
            original_sha256: "77fe93852ccd6706f20de8061f1b4cd04f17a5b761c7517b27df82244e5a72e4",
            patched_sha256: "85ae7c81ed4cedcc171e0bd403bde0aaf9308981ea5ea08b79db58184607000e",
        },
    ],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "FF 83 01 D1 F4 4F 04 A9 FD 7B 05 A9 FD 43 01 91 ?? ?? ?? ?? ?? ?? ?? 97 C0 00 00 37 ?? ?? ?? 97 01 00 80 52 02 00 80 D2 ?? ?? ?? 94 ?? ?? ?? 34",
            offset: 16,
        },
        expected: "?? ?? ?? 97",
        action: Action::Arm64Branch {
            target: Anchor {
                signature: "20 00 80 52 ?? ?? ?? 94 ?? ?? ?? 94 ?? ?? ?? 94 20 00 80 52 FD 7B 45 A9 F4 4F 44 A9 FF 83 01 91 C0 03 5F D6",
                offset: 0,
            },
            link: false,
            verify: Some(44),
        },
    }],
};
