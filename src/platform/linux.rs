use crate::transaction::error;
use std::{
    fs::{self, File, Metadata, OpenOptions},
    io,
    os::{
        fd::AsRawFd,
        unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt},
    },
    path::Path,
};

pub fn options() -> OpenOptions {
    let mut options = OpenOptions::new();
    options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
    options
}
pub fn temporary_options() -> OpenOptions {
    options()
}
pub fn open_source(path: &Path) -> io::Result<File> {
    options().read(true).write(true).open(path)
}
pub fn create_dir(path: &Path) -> io::Result<()> {
    fs::DirBuilder::new().mode(0o700).create(path)
}
pub fn regular(metadata: &Metadata) -> bool {
    metadata.is_file()
}
pub fn directory(metadata: &Metadata) -> bool {
    metadata.is_dir()
}
pub fn busy(error: &io::Error) -> bool {
    error.raw_os_error() == Some(libc::ETXTBSY)
}
pub fn same_file(source: &File, path: &Path) -> io::Result<bool> {
    let original = source.metadata()?;
    let current = fs::symlink_metadata(path)?;
    Ok(regular(&current) && current.dev() == original.dev() && current.ino() == original.ino())
}
pub fn preserve(file: &File, source: &File) -> io::Result<()> {
    let current = file.metadata()?;
    let metadata = source.metadata()?;
    if (current.uid() != metadata.uid() || current.gid() != metadata.gid())
        && unsafe { libc::fchown(file.as_raw_fd(), metadata.uid(), metadata.gid()) } != 0
    {
        return Err(io::Error::last_os_error());
    }
    file.set_permissions(fs::Permissions::from_mode(metadata.mode() & 0o7777))
}
pub fn sync_dir(path: &Path) -> io::Result<()> {
    File::open(path)?.sync_all()
}
pub fn replace(source: &Path, target: &Path) -> io::Result<()> {
    fs::rename(source, target)
}

pub fn ensure_idle(target: &Path) -> Result<(), String> {
    let metadata = fs::metadata(target).map_err(|e| error("cannot inspect executable", e))?;
    for entry in fs::read_dir("/proc")
        .map_err(|e| error("cannot check running processes", e))?
        .flatten()
    {
        if !entry
            .file_name()
            .as_encoded_bytes()
            .iter()
            .all(u8::is_ascii_digit)
        {
            continue;
        }
        let executable = entry.path().join("exe");
        let same_file = fs::metadata(&executable)
            .is_ok_and(|m| m.dev() == metadata.dev() && m.ino() == metadata.ino());
        let resolve = fs::read_link(&executable).is_ok_and(|path| {
            path.file_name().is_some_and(|name| {
                matches!(
                    name.to_str(),
                    Some(
                        "resolve"
                            | "resolve.patched"
                            | "resolve (deleted)"
                            | "resolve.patched (deleted)"
                    )
                )
            })
        });
        if same_file || resolve {
            return Err(format!(
                "resolve is running (pid {}), close it and retry",
                entry.file_name().to_string_lossy()
            ));
        }
    }
    Ok(())
}
