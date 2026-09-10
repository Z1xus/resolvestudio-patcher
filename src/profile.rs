use crate::binary::{Architecture, Platform};
use crate::version::{Version, VersionRange};

pub struct Profile {
    pub id: &'static str,
    pub platform: Platform,
    pub architecture: Architecture,
    pub versions: VersionRange,
    pub builds: &'static [Build],
    pub patches: &'static [Patch],
}

pub struct Build {
    pub version: Version,
    pub original_sha256: &'static str,
    pub patched_sha256: &'static str,
}

impl Profile {
    pub fn build(&self, hash: &str) -> Option<&Build> {
        self.builds
            .iter()
            .find(|build| build.original_sha256 == hash || build.patched_sha256 == hash)
    }
}

pub struct Anchor {
    pub signature: &'static str,
    pub offset: usize,
}

pub struct Rel32 {
    pub displacement_offset: usize,
    pub instruction_end: usize,
}

pub struct Rel8 {
    pub displacement_offset: usize,
    pub instruction_end: usize,
}

pub enum Verify {
    Rel32(Rel32),
    Rel8(Rel8),
}

pub enum Action {
    Bytes(&'static [u8]),
    Code {
        bytes: &'static [u8],
        calls: &'static [Call],
    },
    Jump {
        target: Anchor,
        verify: Option<Verify>,
    },
    Arm64Branch {
        target: Anchor,
        link: bool,
        verify: Option<usize>,
    },
}

pub struct Patch {
    pub name: &'static str,
    pub anchor: Anchor,
    pub expected: &'static str,
    pub action: Action,
}

pub struct Call {
    pub offset: usize,
    pub target: Anchor,
}
