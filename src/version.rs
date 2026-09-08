use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub [u32; 4]);

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [major, minor, patch, build] = self.0;
        write!(f, "{major}.{minor}.{patch}.{build}")
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = text.split('.').collect();
        if parts.len() != 4 {
            return Err("version needs major.minor.patch.build".into());
        }
        let mut values = [0; 4];
        for (value, part) in values.iter_mut().zip(parts) {
            if part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err("invalid version component".into());
            }
            *value = part.parse().map_err(|_| "version component overflow")?;
        }
        Ok(Self(values))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VersionRange {
    pub min: Version,
    pub max: Version,
}

impl VersionRange {
    pub fn contains(&self, version: Version) -> bool {
        self.min <= version && version <= self.max
    }
}

pub fn detect(data: &[u8]) -> Result<Version, String> {
    if data.starts_with(b"MZ") {
        return crate::pe::version(data);
    }
    let suffix = b"_studio\0";
    let mut found = None;
    for (end, bytes) in data.windows(suffix.len()).enumerate() {
        if bytes != suffix {
            continue;
        }
        let start = data[..end]
            .iter()
            .rposition(|&byte| byte == 0)
            .map_or(0, |i| i + 1);
        let Some(version) = std::str::from_utf8(&data[start..end])
            .ok()
            .and_then(|s| s.parse().ok())
        else {
            continue;
        };
        if found.is_some_and(|previous| previous != version) {
            return Err("conflicting embedded versions".into());
        }
        found = Some(version);
    }
    found.ok_or_else(|| "studio version not found".into())
}
