use crate::{elf, pe};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    Linux,
    Windows,
}

impl Platform {
    pub fn is_native(self) -> bool {
        matches!(self, Self::Linux) && cfg!(target_os = "linux")
            || matches!(self, Self::Windows) && cfg!(target_os = "windows")
    }
}

pub struct Segment {
    pub offset: usize,
    pub size: usize,
    pub address: u64,
}

pub fn inspect(data: &[u8]) -> Result<(Platform, Vec<Segment>), String> {
    if data.starts_with(b"MZ") {
        Ok((Platform::Windows, pe::code_segments(data)?))
    } else {
        Ok((Platform::Linux, elf::code_segments(data)?))
    }
}
