use crate::{
    binary::{Architecture, Platform, Segment, Target},
    profile::{Action, Build, Patch, Profile},
    signature::Signature,
    version::{self, Version},
};
use sha2::{Digest, Sha256};

pub fn hash(data: &[u8]) -> String {
    hex(&Sha256::digest(data))
}

pub(crate) fn hex(data: &[u8]) -> String {
    data.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Clone, Copy)]
pub struct Location {
    pub offset: usize,
    pub address: u64,
    end: usize,
}

impl Location {
    fn at(self, offset: usize, width: usize) -> Result<Self, String> {
        let start = self.offset.checked_add(offset).ok_or("offset overflow")?;
        if start.checked_add(width).is_none_or(|end| end > self.end) {
            return Err("patch extends beyond executable segment".into());
        }
        Ok(Self {
            offset: start,
            address: self
                .address
                .checked_add(offset as u64)
                .ok_or("address overflow")?,
            end: self.end,
        })
    }
}

pub fn locate(data: &[u8], segments: &[Segment], pattern: &str) -> Result<Location, String> {
    let pattern = Signature::parse(pattern)?;
    let mut found = None;
    for segment in segments {
        let end = segment
            .offset
            .checked_add(segment.size)
            .ok_or("segment overflow")?;
        let code = data
            .get(segment.offset..end)
            .ok_or("segment outside file")?;
        for offset in pattern.find(code) {
            if found.is_some() {
                return Err("ambiguous signature".into());
            }
            found = Some(Location {
                offset: segment.offset + offset,
                address: segment
                    .address
                    .checked_add(offset as u64)
                    .ok_or("address overflow")?,
                end,
            });
        }
    }
    found.ok_or_else(|| "signature not found".into())
}

pub fn identify(data: &[u8], fingerprint: &str, profiles: &[Profile]) -> Result<Version, String> {
    let versions: Vec<_> = profiles
        .iter()
        .filter_map(|profile| profile.build(fingerprint).map(|build| build.version))
        .collect();
    if let Some(&first) = versions.first() {
        if versions.iter().any(|&version| version != first) {
            return Err("conflicting build records".into());
        }
        return Ok(first);
    }
    version::detect(data)
}

pub struct Change {
    pub name: &'static str,
    pub location: Location,
    pub before: Vec<u8>,
    pub after: Vec<u8>,
    pub applied: bool,
}

pub struct Plan<'a> {
    pub profile: &'a Profile,
    pub build: Option<&'a Build>,
    pub changes: Vec<Change>,
    pub already_patched: bool,
    input_hash: String,
}

fn change(
    data: &[u8],
    segments: &[Segment],
    patch: &Patch,
    architecture: Architecture,
) -> Result<Change, String> {
    let anchor = locate(data, segments, patch.anchor.signature)?;
    let expected = Signature::parse(patch.expected)?;
    let location = anchor.at(patch.anchor.offset, expected.width())?;
    if architecture == Architecture::Arm64
        && (!location.address.is_multiple_of(4) || !expected.width().is_multiple_of(4))
    {
        return Err("arm64 patches must contain complete aligned instructions".into());
    }
    let after = match &patch.action {
        Action::Bytes(bytes) => bytes.to_vec(),
        Action::Code { bytes, calls } => {
            if architecture != Architecture::X86_64 {
                return Err("x86 call relocation on a different architecture".into());
            }
            let mut bytes = bytes.to_vec();
            let mut end = 0;
            for call in *calls {
                let offset = call.offset;
                if offset < end
                    || bytes.get(offset) != Some(&0xe8)
                    || offset.checked_add(5).is_none_or(|n| n > bytes.len())
                {
                    return Err("invalid call relocation".into());
                }
                end = offset + 5;
                let target =
                    locate(data, segments, call.target.signature)?.at(call.target.offset, 1)?;
                let next = location
                    .address
                    .checked_add(end as u64)
                    .ok_or("address overflow")?;
                let displacement = i32::try_from(target.address as i128 - next as i128)
                    .map_err(|_| "call target out of range")?;
                bytes[offset + 1..end].copy_from_slice(&displacement.to_le_bytes());
            }
            bytes
        }
        Action::Jump { target, verify } => {
            if architecture != Architecture::X86_64 {
                return Err("x86 jump on a different architecture".into());
            }
            let target = locate(data, segments, target.signature)?.at(target.offset, 1)?;
            if let Some(reference) = verify {
                let (width, displacement) = match reference {
                    crate::profile::Verify::Rel32(reference) => {
                        let displacement = anchor.at(reference.displacement_offset, 4)?;
                        let displacement = i32::from_le_bytes(
                            data[displacement.offset..displacement.offset + 4]
                                .try_into()
                                .unwrap(),
                        ) as i64;
                        (reference.instruction_end, displacement)
                    }
                    crate::profile::Verify::Rel8(reference) => {
                        let displacement = anchor.at(reference.displacement_offset, 1)?;
                        let displacement = data[displacement.offset] as i8 as i64;
                        (reference.instruction_end, displacement)
                    }
                };
                let next = anchor.at(width, 0)?;
                if next.address.checked_add_signed(displacement) != Some(target.address) {
                    return Err("branch target mismatch".into());
                }
            }
            let next = location.address.checked_add(5).ok_or("address overflow")?;
            let displacement = i32::try_from(target.address as i128 - next as i128)
                .map_err(|_| "jump target out of range")?;
            let mut bytes = vec![0xe9];
            bytes.extend_from_slice(&displacement.to_le_bytes());
            bytes
        }
        Action::Arm64Branch {
            target,
            link,
            verify,
        } => {
            if architecture != Architecture::Arm64 {
                return Err("arm64 branch on a different architecture".into());
            }
            let target = locate(data, segments, target.signature)?.at(target.offset, 4)?;
            if location.address % 4 != 0 || target.address % 4 != 0 {
                return Err("unaligned arm64 branch".into());
            }
            if let Some(offset) = verify {
                let reference = anchor.at(*offset, 4)?;
                if !reference.address.is_multiple_of(4) {
                    return Err("unaligned arm64 branch reference".into());
                }
                let instruction = u32::from_le_bytes(
                    data[reference.offset..reference.offset + 4]
                        .try_into()
                        .unwrap(),
                );
                let (immediate, bits) = if instruction & 0x7c000000 == 0x14000000 {
                    (instruction & 0x03ffffff, 26)
                } else if instruction & 0xff000010 == 0x54000000
                    || instruction & 0x7e000000 == 0x34000000
                {
                    ((instruction >> 5) & 0x7ffff, 19)
                } else if instruction & 0x7e000000 == 0x36000000 {
                    ((instruction >> 5) & 0x3fff, 14)
                } else {
                    return Err("unsupported arm64 branch reference".into());
                };
                let displacement = (((immediate << (32 - bits)) as i32 >> (32 - bits)) as i64) * 4;
                if reference.address.checked_add_signed(displacement) != Some(target.address) {
                    return Err("branch target mismatch".into());
                }
            }
            let displacement = target.address as i128 - location.address as i128;
            if !(-(1i128 << 27)..(1i128 << 27)).contains(&displacement) {
                return Err("arm64 branch target out of range".into());
            }
            let opcode = if *link { 0x94000000u32 } else { 0x14000000u32 };
            (opcode | ((displacement / 4) as u32 & 0x03ffffff))
                .to_le_bytes()
                .to_vec()
        }
    };
    if after.len() != expected.width() {
        return Err("replacement length differs from expected bytes".into());
    }
    let before = data[location.offset..location.offset + after.len()].to_vec();
    let original = expected.matches(&before);
    let applied = before == after;
    if original == applied {
        return Err(if original {
            "original and patched states overlap"
        } else {
            "unexpected bytes"
        }
        .into());
    }
    Ok(Change {
        name: patch.name,
        location,
        before,
        after,
        applied,
    })
}

pub fn plan<'a>(
    data: &[u8],
    segments: &[Segment],
    version: Version,
    fingerprint: &str,
    profiles: &'a [Profile],
    try_profile: Option<&str>,
    target: Target,
) -> Result<Plan<'a>, String> {
    let candidates: Vec<_> = profiles
        .iter()
        .filter(|profile| match try_profile {
            Some(id) => profile.id == id,
            None => profile.versions.contains(version),
        })
        .filter(|profile| {
            profile.platform == target.platform && profile.architecture == target.architecture
        })
        .collect();
    if candidates.is_empty() {
        return Err(match try_profile {
            Some(id) => format!("no {} profile with id: {id}", target.architecture),
            None => format!("no {} profile for {version}", target.architecture),
        });
    }
    let known: Vec<_> = candidates
        .iter()
        .copied()
        .filter(|profile| profile.build(fingerprint).is_some())
        .collect();
    let profile = match known.as_slice() {
        [profile] => *profile,
        [] => {
            if candidates.len() != 1 {
                return Err("overlapping version ranges, profile selection is ambiguous".into());
            }
            candidates[0]
        }
        _ => return Err("duplicate tested build profiles".into()),
    };
    if profile.patches.is_empty() {
        return Err("profile contains no patches".into());
    }
    let mut changes = Vec::new();
    for patch in profile.patches {
        changes.push(
            change(data, segments, patch, target.architecture)
                .map_err(|error| format!("{}: {error}", patch.name))?,
        );
    }
    changes.sort_by_key(|change| change.location.offset);
    for pair in changes.windows(2) {
        if pair[0].location.offset + pair[0].after.len() > pair[1].location.offset {
            return Err("overlapping patches".into());
        }
    }
    let already_patched = changes[0].applied;
    if changes
        .iter()
        .any(|change| change.applied != already_patched)
    {
        return Err("partially patched input".into());
    }
    if target.platform == Platform::Macos
        && let Some(signature) = crate::codesign::ad_hoc(data)?
        && already_patched
        && data[signature.range] != signature.data
    {
        return Err("patched mach-o code signature does not match its contents".into());
    }
    let build = profile.build(fingerprint);
    if let Some(build) = build
        && (build.version != version || already_patched != (fingerprint == build.patched_sha256))
    {
        return Err("profile state disagrees with build record".into());
    }
    Ok(Plan {
        profile,
        build,
        changes,
        already_patched,
        input_hash: fingerprint.into(),
    })
}

impl Plan<'_> {
    pub fn apply(&self, data: &mut [u8]) -> Result<String, String> {
        if self.already_patched {
            return Err("already patched, no output created".into());
        }
        if hash(data) != self.input_hash {
            return Err("input changed after planning".into());
        }
        for change in &self.changes {
            if data.get(change.location.offset..change.location.offset + change.before.len())
                != Some(change.before.as_slice())
            {
                return Err(format!("{}: input bytes changed", change.name));
            }
        }
        for change in &self.changes {
            data[change.location.offset..change.location.offset + change.after.len()]
                .copy_from_slice(&change.after);
        }
        let signature = if self.profile.platform == Platform::Macos {
            match crate::codesign::ad_hoc(data) {
                Ok(signature) => signature.map(
                    |crate::codesign::Signature {
                         range,
                         data: replacement,
                     }| {
                        let original = data[range.clone()].to_vec();
                        data[range.clone()].copy_from_slice(&replacement);
                        (range, original)
                    },
                ),
                Err(error) => {
                    for change in &self.changes {
                        data[change.location.offset..change.location.offset + change.before.len()]
                            .copy_from_slice(&change.before);
                    }
                    return Err(error);
                }
            }
        } else {
            None
        };
        let fingerprint = hash(data);
        if self
            .build
            .is_some_and(|build| fingerprint != build.patched_sha256)
        {
            if let Some((range, original)) = signature {
                data[range].copy_from_slice(&original);
            }
            for change in &self.changes {
                data[change.location.offset..change.location.offset + change.before.len()]
                    .copy_from_slice(&change.before);
            }
            return Err("output hash disagrees with tested build".into());
        }
        Ok(fingerprint)
    }
}
