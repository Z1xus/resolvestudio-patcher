use sha2::{Digest, Sha256, Sha384};
use std::ops::Range;

pub struct Signature {
    pub range: Range<usize>,
    pub data: Vec<u8>,
}

fn word(data: &[u8], offset: usize) -> Result<u32, String> {
    let bytes = data
        .get(offset..offset.checked_add(4).ok_or("signature offset overflow")?)
        .ok_or("truncated code signature")?;
    Ok(u32::from_be_bytes(bytes.try_into().unwrap()))
}

fn put(data: &mut [u8], offset: usize, value: u32) {
    data[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
}

fn region(data: &[u8], offset: usize, length: usize) -> Result<&[u8], String> {
    data.get(
        offset
            ..offset
                .checked_add(length)
                .ok_or("signature range overflow")?,
    )
    .ok_or_else(|| "code signature range is outside its blob".into())
}

fn digest(data: &[u8], kind: u8) -> Result<Vec<u8>, String> {
    Ok(match kind {
        1 => sha1::Sha1::digest(data).to_vec(),
        2 => Sha256::digest(data).to_vec(),
        3 => Sha256::digest(data)[..20].to_vec(),
        4 => Sha384::digest(data).to_vec(),
        _ => return Err("unsupported code signature hash type".into()),
    })
}

fn signature_range(data: &[u8]) -> Result<Option<Range<usize>>, String> {
    let le = |at: usize| -> Result<u32, String> { Ok(word(data, at)?.swap_bytes()) };
    if le(0)? != 0xfeedfacf {
        return Err("expected a thin mach-o64 image for signing".into());
    }
    let commands = region(data, 32, le(20)? as usize)?;
    let mut at = 0;
    let mut found = None;
    for _ in 0..le(16)? {
        let kind = word(commands, at)?.swap_bytes();
        let size = word(commands, at + 4)?.swap_bytes() as usize;
        if size < 8 || !size.is_multiple_of(8) {
            return Err("invalid mach-o load command".into());
        }
        let command = region(commands, at, size)?;
        if kind == 0x1d {
            if found.is_some() || size != 16 {
                return Err("invalid mach-o signature commands".into());
            }
            let offset = word(command, 8)?.swap_bytes() as usize;
            let length = word(command, 12)?.swap_bytes() as usize;
            region(data, offset, length)?;
            if offset < 32 + commands.len() || offset + length != data.len() {
                return Err("mach-o code signature must be at the end of the slice".into());
            }
            found = Some(offset..offset + length);
        }
        at += size;
    }
    if at != commands.len() {
        return Err("mach-o load command size mismatch".into());
    }
    Ok(found)
}

pub fn ad_hoc(data: &[u8]) -> Result<Option<Signature>, String> {
    let Some(file) = signature_range(data)? else {
        return Ok(None);
    };
    let signature = &data[file.clone()];
    if word(signature, 0)? != 0xfade0cc0 {
        return Err("unsupported embedded code signature".into());
    }
    let signature = region(signature, 0, word(signature, 4)? as usize)?;
    let count = word(signature, 8)? as usize;
    let table_end = 12usize
        .checked_add(count.checked_mul(8).ok_or("signature table overflow")?)
        .ok_or("signature table overflow")?;
    region(signature, 0, table_end)?;
    let mut blobs = Vec::new();
    let mut ranges = Vec::new();
    for index in 0..count {
        let slot = word(signature, 12 + index * 8)?;
        let start = word(signature, 16 + index * 8)? as usize;
        let length = word(
            signature,
            start.checked_add(4).ok_or("signature offset overflow")?,
        )? as usize;
        if start < table_end || length < 8 || blobs.iter().any(|(previous, _)| *previous == slot) {
            return Err("invalid code signature blob index".into());
        }
        let blob = region(signature, start, length)?;
        ranges.push(start..start + length);
        blobs.push((slot, blob.to_vec()));
    }
    ranges.sort_by_key(|range| range.start);
    if ranges.windows(2).any(|pair| pair[0].end > pair[1].start) {
        return Err("overlapping code signature blobs".into());
    }
    if !blobs.iter().any(|(slot, _)| *slot == 0) {
        return Err("code directory not found".into());
    }
    blobs.retain(|(slot, _)| !matches!(*slot, 0x10000..=0x10002));
    for (slot, blob) in &mut blobs {
        if *slot == 2 {
            *blob = [0xfade0c01u32, 12, 0]
                .into_iter()
                .flat_map(u32::to_be_bytes)
                .collect();
        }
    }
    let special = blobs
        .iter()
        .filter(|(slot, _)| (1..0x1000).contains(slot))
        .cloned()
        .collect::<Vec<_>>();
    for (slot, directory) in &mut blobs {
        if *slot != 0 && !(0x1000..0x1005).contains(slot) {
            continue;
        }
        if word(directory, 0)? != 0xfade0c02 {
            return Err("invalid code directory magic".into());
        }
        let version = word(directory, 8)?;
        let header_size = match version {
            0x20000 => 44,
            0x20100 => 48,
            0x20200 => 52,
            0x20300 => 64,
            0x20400 => 88,
            0x20500 => 96,
            0x20600 => 108,
            _ => return Err("unsupported code directory version".into()),
        };
        region(directory, 0, header_size)?;
        if version >= 0x20100 && word(directory, 44)? != 0
            || version >= 0x20500 && word(directory, 92)? != 0
            || version >= 0x20600 && word(directory, 104)? != 0
        {
            return Err("scatter, encrypted or linked code signatures are unsupported".into());
        }
        let hash_offset = word(directory, 16)? as usize;
        let special_count = word(directory, 24)? as usize;
        let code_count = word(directory, 28)? as usize;
        let mut limit = word(directory, 32)? as u64;
        if version >= 0x20300 {
            let limit64 = (word(directory, 56)? as u64) << 32 | word(directory, 60)? as u64;
            if limit64 != 0 {
                limit = limit64;
            }
        }
        if limit != file.start as u64 {
            return Err("code directory does not cover the complete image".into());
        }
        let hash_size = directory[36] as usize;
        let kind = directory[37];
        if hash_size != digest(&[], kind)?.len() {
            return Err("invalid code directory hash size".into());
        }
        let page = directory[39];
        let page_size = if page == 0 {
            file.start
        } else {
            1usize
                .checked_shl(page as u32)
                .ok_or("invalid code signature page size")?
        };
        if page_size == 0 || code_count != file.start.div_ceil(page_size) {
            return Err("invalid code directory page count".into());
        }
        let hash_start = hash_offset
            .checked_sub(
                special_count
                    .checked_mul(hash_size)
                    .ok_or("signature hash overflow")?,
            )
            .ok_or("invalid special hash slots")?;
        if hash_start < header_size {
            return Err("code directory hashes overlap its header".into());
        }
        region(
            directory,
            hash_offset,
            code_count
                .checked_mul(hash_size)
                .ok_or("signature hash overflow")?,
        )?;
        let identifier = word(directory, 20)? as usize;
        if identifier < header_size
            || identifier >= hash_start
            || !directory[identifier..hash_start].contains(&0)
        {
            return Err("invalid code signature identifier".into());
        }
        let flags = (word(directory, 12)? | 2) & !0x20000;
        put(directory, 12, flags);
        if version >= 0x20200 {
            put(directory, 48, 0);
        }
        for (index, page) in data[..file.start].chunks(page_size).enumerate() {
            let start = hash_offset + index * hash_size;
            directory[start..start + hash_size].copy_from_slice(&digest(page, kind)?);
        }
        for (slot, blob) in &special {
            if *slot as usize > special_count {
                return Err("embedded signature blob has no special hash slot".into());
            }
            let start = hash_offset - *slot as usize * hash_size;
            directory[start..start + hash_size].copy_from_slice(&digest(blob, kind)?);
        }
    }
    let mut output = vec![0; file.len()];
    let mut cursor = 12 + blobs.len() * 8;
    for (index, (slot, blob)) in blobs.iter().enumerate() {
        region(&output, cursor, blob.len())?;
        put(&mut output, 12 + index * 8, *slot);
        put(&mut output, 16 + index * 8, cursor as u32);
        output[cursor..cursor + blob.len()].copy_from_slice(blob);
        cursor += blob.len();
    }
    put(&mut output, 0, 0xfade0cc0);
    put(&mut output, 4, cursor as u32);
    put(&mut output, 8, blobs.len() as u32);
    Ok(Some(Signature {
        range: file,
        data: output,
    }))
}
