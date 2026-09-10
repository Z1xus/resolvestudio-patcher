use crate::transaction::error;
use std::{
    ffi::OsStr,
    fs, io,
    os::unix::{ffi::OsStrExt, fs::MetadataExt},
    path::Path,
};

pub fn ensure_idle(target: &Path) -> Result<(), String> {
    let metadata = fs::metadata(target).map_err(|e| error("cannot inspect executable", e))?;
    let mut capacity = 1024;
    let pids = loop {
        let mut pids = vec![0i32; capacity];
        let length = unsafe {
            libc::proc_listpids(
                1,
                0,
                pids.as_mut_ptr().cast(),
                (capacity * size_of::<i32>()) as i32,
            )
        };
        if length <= 0 {
            return Err(error(
                "cannot check running processes",
                io::Error::last_os_error(),
            ));
        }
        let count = length as usize / size_of::<i32>();
        if count < capacity {
            pids.truncate(count);
            break pids;
        }
        if capacity >= 1_048_576 {
            return Err("process list is too large".into());
        }
        capacity *= 2;
    };
    for pid in pids.into_iter().filter(|pid| *pid > 0) {
        let mut buffer = [0u8; libc::PROC_PIDPATHINFO_MAXSIZE as usize];
        let length =
            unsafe { libc::proc_pidpath(pid, buffer.as_mut_ptr().cast(), buffer.len() as u32) };
        let mut same_file = false;
        let name = if length > 0 {
            let end = buffer
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(buffer.len());
            let path = Path::new(OsStr::from_bytes(&buffer[..end]));
            same_file = fs::metadata(path)
                .is_ok_and(|m| m.dev() == metadata.dev() && m.ino() == metadata.ino());
            path.file_name().unwrap_or_default().to_os_string()
        } else {
            let length =
                unsafe { libc::proc_name(pid, buffer.as_mut_ptr().cast(), buffer.len() as u32) };
            if length <= 0 {
                continue;
            }
            let end = buffer
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(buffer.len());
            OsStr::from_bytes(&buffer[..end]).to_os_string()
        };
        if same_file
            || name.to_str().is_some_and(|name| {
                name.eq_ignore_ascii_case("resolve") || name.eq_ignore_ascii_case("resolve.patched")
            })
        {
            return Err(format!(
                "resolve is running (pid {pid}), close it and retry"
            ));
        }
    }
    Ok(())
}
