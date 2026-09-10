use crate::{elf, macho, pe};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    Linux,
    Windows,
    Macos,
}

impl Platform {
    pub fn is_native(self) -> bool {
        matches!(self, Self::Linux) && cfg!(target_os = "linux")
            || matches!(self, Self::Windows) && cfg!(target_os = "windows")
            || matches!(self, Self::Macos) && cfg!(target_os = "macos")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    Arm64,
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::X86_64 => "x86-64",
            Self::Arm64 => "arm64",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target {
    pub platform: Platform,
    pub architecture: Architecture,
}

pub struct Segment {
    pub offset: usize,
    pub size: usize,
    pub address: u64,
}

pub struct Slice {
    pub target: Target,
    pub offset: usize,
    pub size: usize,
    pub segments: Vec<Segment>,
}

pub fn inspect(data: &[u8]) -> Result<Vec<Slice>, String> {
    let (platform, segments) = if data.starts_with(b"MZ") {
        (Platform::Windows, pe::code_segments(data)?)
    } else if data.starts_with(b"\x7fELF") {
        (Platform::Linux, elf::code_segments(data)?)
    } else if macho::is_macho(data) {
        return macho::slices(data);
    } else {
        return Err("expected an elf, pe or mach-o executable".into());
    };
    Ok(vec![Slice {
        target: Target {
            platform,
            architecture: Architecture::X86_64,
        },
        offset: 0,
        size: data.len(),
        segments,
    }])
}
