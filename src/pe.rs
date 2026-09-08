use crate::{binary::Segment, version::Version};

fn bytes<const N: usize>(data: &[u8], offset: usize) -> Result<[u8; N], String> {
    data.get(offset..offset.checked_add(N).ok_or("pe offset overflow")?)
        .ok_or_else(|| "truncated pe structure".into())
        .map(|value| value.try_into().unwrap())
}

fn word(data: &[u8], offset: usize) -> Result<u16, String> {
    Ok(u16::from_le_bytes(bytes(data, offset)?))
}

fn dword(data: &[u8], offset: usize) -> Result<u32, String> {
    Ok(u32::from_le_bytes(bytes(data, offset)?))
}

struct Image {
    sections: Vec<(Segment, u32)>,
    resources: (u32, u32),
    base: u64,
}

impl Image {
    fn read(data: &[u8]) -> Result<Self, String> {
        if data.get(..2) != Some(b"MZ") {
            return Err("expected a pe executable".into());
        }
        let header = dword(data, 0x3c)? as usize;
        if bytes::<4>(data, header)? != *b"PE\0\0" || word(data, header + 4)? != 0x8664 {
            return Err("expected an x86-64 pe executable".into());
        }
        let count = word(data, header + 6)? as usize;
        let optional = header + 24;
        let size = word(data, header + 20)? as usize;
        if count == 0
            || count > 96
            || size < 136
            || word(data, optional)? != 0x20b
            || word(data, header + 22)? & 0x2002 != 2
            || dword(data, optional + 108)? < 3
        {
            return Err("unsupported pe header".into());
        }
        let base = u64::from_le_bytes(bytes(data, optional + 24)?);
        let table = optional + size;
        let headers_end = table + count * 40;
        if headers_end > data.len() {
            return Err("truncated pe section table".into());
        }
        let mut sections = Vec::new();
        for index in 0..count {
            let section = table + index * 40;
            let rva = dword(data, section + 12)? as u64;
            let size = dword(data, section + 16)? as usize;
            let offset = dword(data, section + 20)? as usize;
            if size == 0 {
                continue;
            }
            if offset < headers_end || offset.checked_add(size).is_none_or(|end| end > data.len()) {
                return Err("invalid pe section bounds".into());
            }
            let address = base.checked_add(rva).ok_or("pe address overflow")?;
            address
                .checked_add(size as u64)
                .ok_or("pe address overflow")?;
            sections.push((
                Segment {
                    offset,
                    size,
                    address,
                },
                dword(data, section + 36)?,
            ));
        }
        sections.sort_by_key(|(section, _)| section.offset);
        for pair in sections.windows(2) {
            if pair[0].0.offset + pair[0].0.size > pair[1].0.offset {
                return Err("overlapping pe file sections".into());
            }
        }
        for (index, (a, _)) in sections.iter().enumerate() {
            for (b, _) in &sections[index + 1..] {
                if a.address < b.address + b.size as u64 && b.address < a.address + a.size as u64 {
                    return Err("overlapping pe virtual sections".into());
                }
            }
        }
        Ok(Self {
            sections,
            resources: (dword(data, optional + 128)?, dword(data, optional + 132)?),
            base,
        })
    }

    fn offset(&self, rva: u32, size: usize) -> Result<usize, String> {
        let address = self
            .base
            .checked_add(rva as u64)
            .ok_or("pe address overflow")?;
        for (section, _) in &self.sections {
            if let Some(delta) = address.checked_sub(section.address)
                && delta
                    .checked_add(size as u64)
                    .is_some_and(|end| end <= section.size as u64)
            {
                return Ok(section.offset + delta as usize);
            }
        }
        Err("pe resource outside file sections".into())
    }
}

pub fn code_segments(data: &[u8]) -> Result<Vec<Segment>, String> {
    let segments: Vec<_> = Image::read(data)?
        .sections
        .into_iter()
        .filter(|(_, flags)| flags & 0x20000000 != 0)
        .map(|(segment, _)| segment)
        .collect();
    if segments.is_empty() {
        return Err("pe has no executable sections".into());
    }
    Ok(segments)
}

pub fn version(data: &[u8]) -> Result<Version, String> {
    if !data
        .windows(23)
        .any(|bytes| bytes == b"DaVinci Resolve Studio\0")
    {
        return Err("studio product marker not found".into());
    }
    let image = Image::read(data)?;
    let (rva, size) = image.resources;
    let start = image.offset(rva, size as usize)?;
    let resources = &data[start..start + size as usize];
    let mut directories = vec![0];
    for depth in 0..3 {
        let mut next = Vec::new();
        for directory in directories {
            let count = word(resources, directory + 12)? as usize
                + word(resources, directory + 14)? as usize;
            for index in 0..count {
                let entry = directory + 16 + index * 8;
                let id = dword(resources, entry)?;
                let target = dword(resources, entry + 4)?;
                if depth == 0 && id != 16 {
                    continue;
                }
                if (target & 0x80000000 != 0) != (depth < 2) {
                    return Err("invalid pe version resource tree".into());
                }
                next.push((target & 0x7fffffff) as usize);
                if next.len() > 1024 {
                    return Err("too many pe version resources".into());
                }
            }
        }
        directories = next;
    }
    let mut found = None;
    for entry in directories {
        let offset = image.offset(
            dword(resources, entry)?,
            dword(resources, entry + 4)? as usize,
        )?;
        let length = word(data, offset)? as usize;
        if length > dword(resources, entry + 4)? as usize {
            return Err("version info exceeds resource bounds".into());
        }
        let value = data
            .get(offset..offset + length)
            .ok_or("truncated version resource")?;
        let key: Vec<_> = "VS_VERSION_INFO\0"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect();
        if value.get(6..6 + key.len()) != Some(key.as_slice())
            || word(value, 2)? != 52
            || dword(value, 40)? != 0xfeef04bd
            || dword(value, 44)? != 0x10000
        {
            return Err("invalid pe version info".into());
        }
        let high = dword(value, 48)?;
        let low = dword(value, 52)?;
        let version = Version([high >> 16, high & 0xffff, low >> 16, low & 0xffff]);
        if found.is_some_and(|previous| previous != version) {
            return Err("conflicting pe versions".into());
        }
        found = Some(version);
    }
    found.ok_or_else(|| "pe version resource not found".into())
}
