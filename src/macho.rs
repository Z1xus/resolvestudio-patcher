use crate::binary::{Architecture, Platform, Segment, Slice, Target};

const MH_MAGIC_64: u32 = 0xfeedfacf;
const FAT_MAGIC: u32 = 0xcafebabe;
const FAT_MAGIC_64: u32 = 0xcafebabf;

fn bytes<const N: usize>(data: &[u8], offset: usize) -> Result<[u8; N], String> {
    data.get(offset..offset.checked_add(N).ok_or("mach-o offset overflow")?)
        .ok_or_else(|| "truncated mach-o structure".into())
        .map(|value| value.try_into().unwrap())
}

fn u32_at(data: &[u8], offset: usize) -> Result<u32, String> {
    Ok(u32::from_le_bytes(bytes(data, offset)?))
}

fn u64_at(data: &[u8], offset: usize) -> Result<u64, String> {
    Ok(u64::from_le_bytes(bytes(data, offset)?))
}

fn range(offset: u64, size: u64, limit: usize) -> Result<std::ops::Range<usize>, String> {
    let end = offset.checked_add(size).ok_or("mach-o range overflow")?;
    if end > limit as u64 {
        return Err("mach-o range is outside the file".into());
    }
    Ok(offset as usize..end as usize)
}

pub fn is_macho(data: &[u8]) -> bool {
    matches!(
        u32_at(data, 0),
        Ok(0xfeedface
            | 0xcefaedfe
            | MH_MAGIC_64
            | 0xcffaedfe
            | FAT_MAGIC
            | 0xbebafeca
            | FAT_MAGIC_64
            | 0xbfbafeca)
    )
}

fn thin(data: &[u8], offset: usize) -> Result<Slice, String> {
    if u32_at(data, 0)? != MH_MAGIC_64 || data.len() < 32 {
        return Err("expected a little-endian mach-o64 image".into());
    }
    let architecture = match u32_at(data, 4)? {
        0x01000007 => Architecture::X86_64,
        0x0100000c => Architecture::Arm64,
        _ => return Err("expected an x86-64 or arm64 mach-o image".into()),
    };
    if !matches!(u32_at(data, 12)?, 2 | 6 | 8) {
        return Err("expected a mach-o executable, dylib or bundle".into());
    }
    let count = u32_at(data, 16)? as usize;
    let commands = range(32, u32_at(data, 20)? as u64, data.len())?;
    if count == 0 || count > commands.len() / 8 {
        return Err("invalid mach-o load command count".into());
    }
    let mut cursor = commands.start;
    let mut segments = Vec::new();
    let mut mapped = Vec::new();
    for _ in 0..count {
        let header = data
            .get(cursor..commands.end)
            .ok_or("truncated mach-o load command")?;
        let command = u32_at(header, 0)?;
        let size = u32_at(header, 4)? as usize;
        if size < 8 || !size.is_multiple_of(8) || size > header.len() {
            return Err("invalid mach-o load command size".into());
        }
        let header = &header[..size];
        match command {
            0x19 => {
                let address = u64_at(header, 24)?;
                let memory_size = u64_at(header, 32)?;
                let memory_end = address
                    .checked_add(memory_size)
                    .ok_or("mach-o address overflow")?;
                let file = range(u64_at(header, 40)?, u64_at(header, 48)?, data.len())?;
                let executable = u32_at(header, 60)? & 4 != 0;
                let sections = u32_at(header, 64)? as usize;
                if size < 72
                    || sections != (size - 72) / 80
                    || !(size - 72).is_multiple_of(80)
                    || file.len() as u64 > memory_size
                {
                    return Err("invalid mach-o segment layout".into());
                }
                mapped.push((file.clone(), address..memory_end));
                for index in 0..sections {
                    let section = &header[72 + index * 80..72 + (index + 1) * 80];
                    let start = u64_at(section, 32)?;
                    let length = u64_at(section, 40)?;
                    let end = start
                        .checked_add(length)
                        .ok_or("mach-o section address overflow")?;
                    if start < address || end > memory_end || section[16..32] != header[8..24] {
                        return Err("mach-o section is outside its segment".into());
                    }
                    let flags = u32_at(section, 64)?;
                    let instructions = flags & 0x80000400 != 0;
                    if matches!(flags & 0xff, 1 | 0xc | 0x12) {
                        if instructions {
                            return Err("mach-o instructions have no file data".into());
                        }
                        continue;
                    }
                    if length == 0 {
                        continue;
                    }
                    let section_file = range(u32_at(section, 48)? as u64, length, data.len())?;
                    if section_file.start < commands.end
                        || section_file.start < file.start
                        || section_file.end > file.end
                        || start - address != (section_file.start - file.start) as u64
                    {
                        return Err("invalid mach-o section file mapping".into());
                    }
                    if executable && instructions {
                        segments.push(Segment {
                            offset: section_file.start,
                            size: section_file.len(),
                            address: start,
                        });
                    }
                }
            }
            0x21 | 0x2c => {
                if size < if command == 0x2c { 24 } else { 20 } {
                    return Err("truncated mach-o encryption command".into());
                }
                range(
                    u32_at(header, 8)? as u64,
                    u32_at(header, 12)? as u64,
                    data.len(),
                )?;
                if u32_at(header, 16)? != 0 {
                    return Err("encrypted mach-o images are unsupported".into());
                }
            }
            0x1d => {
                if size != 16 {
                    return Err("invalid mach-o code signature command".into());
                }
                range(
                    u32_at(header, 8)? as u64,
                    u32_at(header, 12)? as u64,
                    data.len(),
                )?;
            }
            0x32 => {
                if size < 24 || u32_at(header, 20)? as u64 * 8 + 24 != size as u64 {
                    return Err("invalid mach-o build version command".into());
                }
                if u32_at(header, 8)? != 1 {
                    return Err("expected a macos mach-o image".into());
                }
            }
            0x25 | 0x2f | 0x30 => return Err("expected a macos mach-o image".into()),
            _ => {}
        }
        cursor += size;
    }
    if cursor != commands.end {
        return Err("mach-o load command size mismatch".into());
    }
    for (index, (file, memory)) in mapped.iter().enumerate() {
        for (other_file, other_memory) in &mapped[index + 1..] {
            if (!file.is_empty()
                && !other_file.is_empty()
                && file.start < other_file.end
                && other_file.start < file.end)
                || (!memory.is_empty()
                    && !other_memory.is_empty()
                    && memory.start < other_memory.end
                    && other_memory.start < memory.end)
            {
                return Err("overlapping mach-o segments".into());
            }
        }
    }
    segments.sort_by_key(|segment| segment.offset);
    for pair in segments.windows(2) {
        if pair[0].offset + pair[0].size > pair[1].offset {
            return Err("overlapping mach-o instruction sections".into());
        }
    }
    if segments.is_empty() {
        return Err("mach-o has no executable instruction sections".into());
    }
    Ok(Slice {
        target: Target {
            platform: Platform::Macos,
            architecture,
        },
        offset,
        size: data.len(),
        segments,
    })
}

pub fn slices(data: &[u8]) -> Result<Vec<Slice>, String> {
    let magic = u32::from_be_bytes(bytes(data, 0)?);
    let (wide, swapped) = match magic {
        FAT_MAGIC => (false, false),
        FAT_MAGIC_64 => (true, false),
        0xbebafeca => (false, true),
        0xbfbafeca => (true, true),
        _ => return Ok(vec![thin(data, 0)?]),
    };
    let word = |offset| -> Result<u32, String> {
        let value = bytes(data, offset)?;
        Ok(if swapped {
            u32::from_le_bytes(value)
        } else {
            u32::from_be_bytes(value)
        })
    };
    let long = |offset| -> Result<u64, String> {
        let value = bytes(data, offset)?;
        Ok(if swapped {
            u64::from_le_bytes(value)
        } else {
            u64::from_be_bytes(value)
        })
    };
    let count = word(4)? as usize;
    let entry_size = if wide { 32 } else { 20 };
    let table = range(8, count as u64 * entry_size as u64, data.len())?;
    if count == 0 {
        return Err("mach-o universal file has no slices".into());
    }
    let mut records = Vec::new();
    for index in 0..count {
        let entry = 8 + index * entry_size;
        let cpu = (word(entry)?, word(entry + 4)?);
        let (offset, size, align) = if wide {
            if word(entry + 28)? != 0 {
                return Err("invalid mach-o fat64 reserved field".into());
            }
            (long(entry + 8)?, long(entry + 16)?, word(entry + 24)?)
        } else {
            (
                word(entry + 8)? as u64,
                word(entry + 12)? as u64,
                word(entry + 16)?,
            )
        };
        let file = range(offset, size, data.len())?;
        if file.start < table.end || size < 32 || align > 63 || offset & ((1u64 << align) - 1) != 0
        {
            return Err("invalid mach-o universal slice bounds or alignment".into());
        }
        if records.iter().any(|(previous, _)| *previous == cpu) {
            return Err("duplicate mach-o universal architecture".into());
        }
        records.push((cpu, file));
    }
    records.sort_by_key(|(_, file)| file.start);
    for pair in records.windows(2) {
        if pair[0].1.end > pair[1].1.start {
            return Err("overlapping mach-o universal slices".into());
        }
    }
    records
        .into_iter()
        .map(|(cpu, file)| {
            let image = &data[file.clone()];
            if (u32_at(image, 4)?, u32_at(image, 8)?) != cpu {
                return Err("mach-o slice architecture disagrees with universal header".into());
            }
            thin(image, file.start)
        })
        .collect()
}

pub fn code_segments(data: &[u8]) -> Result<Vec<Segment>, String> {
    let mut segments = Vec::new();
    for slice in slices(data)? {
        for mut segment in slice.segments {
            segment.offset += slice.offset;
            segments.push(segment);
        }
    }
    Ok(segments)
}
