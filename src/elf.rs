use crate::binary::Segment;

fn bytes<const N: usize>(data: &[u8], offset: usize) -> Result<[u8; N], String> {
    let end = offset.checked_add(N).ok_or("elf offset overflow")?;
    data.get(offset..end)
        .ok_or_else(|| "truncated elf structure".into())
        .map(|value| value.try_into().unwrap())
}

fn u16_at(data: &[u8], offset: usize) -> Result<u16, String> {
    Ok(u16::from_le_bytes(bytes(data, offset)?))
}

fn u32_at(data: &[u8], offset: usize) -> Result<u32, String> {
    Ok(u32::from_le_bytes(bytes(data, offset)?))
}

fn u64_at(data: &[u8], offset: usize) -> Result<u64, String> {
    Ok(u64::from_le_bytes(bytes(data, offset)?))
}

pub fn code_segments(data: &[u8]) -> Result<Vec<Segment>, String> {
    if data.len() < 64 || data.get(..7) != Some(b"\x7fELF\x02\x01\x01") {
        return Err("expected a little-endian elf64 file".into());
    }
    if !matches!(u16_at(data, 16)?, 2 | 3) || u16_at(data, 18)? != 62 {
        return Err("expected an x86-64 executable or shared object".into());
    }
    if u32_at(data, 20)? != 1 || u16_at(data, 52)? != 64 {
        return Err("unsupported elf header".into());
    }
    let table = u64_at(data, 32)? as usize;
    let entry_size = u16_at(data, 54)? as usize;
    let count = u16_at(data, 56)? as usize;
    if entry_size != 56 || count == 0 || count == 0xffff {
        return Err("unsupported elf program header table".into());
    }
    let end = table
        .checked_add(entry_size * count)
        .ok_or("elf table overflow")?;
    if table < 64 || end > data.len() {
        return Err("elf program header table is outside the file".into());
    }
    let mut segments = Vec::new();
    for index in 0..count {
        let header = table + index * entry_size;
        let loadable = u32_at(data, header)? == 1;
        let executable = u32_at(data, header + 4)? & 1 != 0;
        if !loadable || !executable {
            continue;
        }
        let offset = u64_at(data, header + 8)? as usize;
        let address = u64_at(data, header + 16)?;
        let size = u64_at(data, header + 32)? as usize;
        let memory_size = u64_at(data, header + 40)?;
        let end = offset.checked_add(size).ok_or("elf segment overflow")?;
        if end > data.len() || size as u64 > memory_size {
            return Err("invalid elf executable segment bounds".into());
        }
        address
            .checked_add(memory_size)
            .ok_or("elf virtual address overflow")?;
        if size != 0 {
            segments.push(Segment {
                offset,
                size,
                address,
            });
        }
    }
    if segments.is_empty() {
        return Err("elf has no executable file segments".into());
    }
    segments.sort_by_key(|segment| segment.offset);
    for pair in segments.windows(2) {
        if pair[0].offset + pair[0].size > pair[1].offset {
            return Err("overlapping executable file segments are unsupported".into());
        }
    }
    Ok(segments)
}
