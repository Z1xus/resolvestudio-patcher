use crate::{
    binary::{Architecture, Platform},
    profile::{Action, Anchor, Build, Patch, Profile, Rel8, Verify},
    version::{Version, VersionRange},
};

const VERSIONS: VersionRange = VersionRange {
    min: Version([19, 0, 0, 0]),
    max: Version([19, u32::MAX, u32::MAX, u32::MAX]),
};

pub const X86_64: Profile = Profile {
    id: "macos-19",
    platform: Platform::Macos,
    architecture: Architecture::X86_64,
    versions: VERSIONS,
    builds: &[
        Build {
            version: Version([19, 0, 0, 69]),
            original_sha256: "a762bdafc8c8e75945949ff4fdfce69e53597ef32771a6c94b1deb6792af0ba5",
            patched_sha256: "4f6f6e97a6a95c345f6a9d79d14bded6008bfbd1b3d77d9416b725928f080479",
        },
        Build {
            version: Version([19, 1, 4, 11]),
            original_sha256: "e4e249c0511778c94fde955184c1e94ea36940f800480d53dc7d86b1272f67ae",
            patched_sha256: "72a6f92b24fda0629da16db3867977bc5693b931a096c373ba9130d09972a430",
        },
    ],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "55 48 89 E5 53 48 83 EC 48 ?? ?? ?? ?? ?? 48 89 C7 31 F6 31 D2 E8 ?? ?? ?? ?? 85 C0 74 ??",
            offset: 9,
        },
        expected: "E8 ?? ?? ?? ??",
        action: Action::Jump {
            target: Anchor {
                signature: "E8 ?? ?? ?? ?? 48 89 C7 E8 ?? ?? ?? ?? B0 01 E9 ?? ?? ?? ?? 48 8D 05",
                offset: 0,
            },
            verify: Some(Verify::Rel8(Rel8 {
                displacement_offset: 29,
                instruction_end: 30,
            })),
        },
    }],
};

pub const ARM64: Profile = Profile {
    id: "macos-19",
    platform: Platform::Macos,
    architecture: Architecture::Arm64,
    versions: VERSIONS,
    builds: &[
        Build {
            version: Version([19, 0, 0, 69]),
            original_sha256: "eee3a2210c8f95eab8868306261c906bfa26ae483aa6c73e3ca6ee8031497caa",
            patched_sha256: "6a0da0ae0cee07a354c07867f52ab2d2df42fc33784935ce074436322d95a3c7",
        },
        Build {
            version: Version([19, 1, 4, 11]),
            original_sha256: "0535fa4173bbba099031e972c954901284f7bd34155edf020db63d6bf5d9d9e1",
            patched_sha256: "35067e1ecd8550b9c2d942689971a650f42e907534699b4823eb0bbf17f02c0c",
        },
    ],
    patches: &[Patch {
        name: "startup",
        anchor: Anchor {
            signature: "FD 7B 05 A9 FD 43 01 91 ?? ?? ?? ?? 01 00 80 52 02 00 80 D2 ?? ?? ?? 94 ?? ?? ?? 34",
            offset: 8,
        },
        expected: "?? ?? ?? 97",
        action: Action::Arm64Branch {
            target: Anchor {
                signature: "?? ?? ?? 94 ?? ?? ?? 94 20 00 80 52 29 00 00 14",
                offset: 0,
            },
            link: false,
            verify: Some(24),
        },
    }],
};
