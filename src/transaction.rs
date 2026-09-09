use crate::{engine, platform};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{self, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub(crate) fn error(context: &str, error: io::Error) -> String {
    if error.kind() == io::ErrorKind::PermissionDenied {
        format!("permission denied: {context}")
    } else if platform::busy(&error) {
        "file is in use, close resolve and retry".into()
    } else {
        format!("{context}: {}", error.to_string().to_lowercase())
    }
}

pub use crate::platform::ensure_idle;

fn digest(file: &mut File) -> Result<String, String> {
    file.seek(SeekFrom::Start(0))
        .map_err(|e| error("cannot seek file", e))?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|e| error("cannot read file", e))?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(engine::hex(&hash.finalize()))
}

fn sync_dir(path: &Path) -> Result<(), String> {
    platform::sync_dir(path).map_err(|e| error("cannot sync directory", e))
}

struct Staged {
    path: PathBuf,
    file: File,
}

impl Staged {
    fn new(
        parent: &Path,
        reader: &mut impl Read,
        source: Option<&File>,
        expected: &str,
    ) -> Result<Self, String> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos();
        let path = parent.join(format!(".patcher-{}-{nonce}.tmp", std::process::id()));
        let file = platform::temporary_options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| error("cannot create temporary file", e))?;
        let mut staged = Self { path, file };
        io::copy(reader, &mut staged.file).map_err(|e| error("cannot write temporary file", e))?;
        if let Some(source) = source {
            platform::preserve(&staged.file, source)
                .map_err(|e| error("cannot preserve file permissions", e))?;
        }
        staged
            .file
            .sync_all()
            .map_err(|e| error("cannot sync temporary file", e))?;
        if digest(&mut staged.file)? != expected {
            return Err("temporary file verification failed".into());
        }
        Ok(staged)
    }

    fn replace(&self, target: &Path) -> Result<(), String> {
        platform::replace(&self.path, target).map_err(|e| error("cannot replace executable", e))?;
        sync_dir(target.parent().ok_or("output has no parent directory")?)
    }
}

impl Drop for Staged {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

pub struct Transaction {
    target: PathBuf,
    pub backups: PathBuf,
    source: Option<File>,
    _lock: File,
}

impl Transaction {
    pub fn open(path: &Path) -> Result<Self, String> {
        let metadata =
            fs::symlink_metadata(path).map_err(|e| error("cannot inspect executable", e))?;
        if !platform::regular(&metadata) {
            return Err("target must be a regular file, symlinks are not supported".into());
        }
        let target = fs::canonicalize(path).map_err(|e| error("cannot resolve path", e))?;
        ensure_idle(&target)?;
        let source = platform::open_source(&target)
            .map_err(|e| error("cannot open executable for replacement", e))?;
        let mut name = target
            .file_name()
            .ok_or("target has no filename")?
            .to_os_string();
        name.push(".backups");
        let backups = target.with_file_name(name);
        match platform::create_dir(&backups) {
            Ok(()) => sync_dir(target.parent().unwrap())?,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(error("cannot create backup directory", e)),
        }
        if !platform::directory(
            &fs::symlink_metadata(&backups)
                .map_err(|e| error("cannot inspect backup directory", e))?,
        ) {
            return Err("backup path must be a directory, symlinks are not supported".into());
        }
        let lock = platform::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(backups.join("lock"))
            .map_err(|e| error("cannot open backup lock", e))?;
        lock.try_lock()
            .map_err(|_| "another patch, restore or cleanup is in progress")?;
        let transaction = Self {
            target,
            backups,
            source: Some(source),
            _lock: lock,
        };
        transaction.verify_path()?;
        Ok(transaction)
    }

    fn verify_path(&self) -> Result<(), String> {
        if !platform::same_file(
            self.source.as_ref().ok_or("transaction is complete")?,
            &self.target,
        )
        .map_err(|e| error("cannot inspect executable", e))?
        {
            return Err("executable changed during operation".into());
        }
        Ok(())
    }

    pub fn read(&mut self) -> Result<Vec<u8>, String> {
        self.source
            .as_mut()
            .ok_or("transaction is complete")?
            .seek(SeekFrom::Start(0))
            .map_err(|e| error("cannot seek executable", e))?;
        let mut data = Vec::new();
        self.source
            .as_mut()
            .ok_or("transaction is complete")?
            .read_to_end(&mut data)
            .map_err(|e| error("cannot read executable", e))?;
        Ok(data)
    }

    fn backup(&mut self, expected: &str) -> Result<(), String> {
        let target = self.backups.join(format!("{expected}.bak"));
        if target
            .try_exists()
            .map_err(|e| error("cannot inspect backup", e))?
        {
            let mut file = platform::options()
                .read(true)
                .open(&target)
                .map_err(|e| error("cannot open backup", e))?;
            if digest(&mut file)? != expected {
                return Err("existing backup is damaged".into());
            }
        } else {
            self.source
                .as_mut()
                .ok_or("transaction is complete")?
                .seek(SeekFrom::Start(0))
                .map_err(|e| error("cannot seek executable", e))?;
            let metadata_source = self
                .source
                .as_ref()
                .ok_or("transaction is complete")?
                .try_clone()
                .map_err(|e| error("cannot inspect executable", e))?;
            let staged = Staged::new(
                &self.backups,
                self.source.as_mut().ok_or("transaction is complete")?,
                Some(&metadata_source),
                expected,
            )?;
            fs::hard_link(&staged.path, &target).map_err(|e| error("cannot publish backup", e))?;
            sync_dir(&self.backups)?;
        }
        Ok(())
    }

    fn replace(&mut self, staged: &Staged, original: &str) -> Result<(), String> {
        ensure_idle(&self.target)?;
        self.verify_path()?;
        if digest(self.source.as_mut().ok_or("transaction is complete")?)? != original {
            return Err("executable content changed during operation".into());
        }
        #[cfg(windows)]
        drop(self.source.take());
        staged.replace(&self.target)
    }

    pub fn patch(&mut self, data: &[u8], original: &str, patched: &str) -> Result<(), String> {
        self.backup(original)?;
        let staged = Staged::new(
            self.target.parent().unwrap(),
            &mut &data[..],
            self.source.as_ref(),
            patched,
        )?;
        let state = format!("{original}\n{patched}\n");
        let statefile = Staged::new(
            &self.backups,
            &mut state.as_bytes(),
            None,
            &engine::hash(state.as_bytes()),
        )?;
        statefile.replace(&self.backups.join("state"))?;
        self.replace(&staged, original)
    }

    pub fn cleanup(
        &mut self,
        confirm: impl FnOnce(u64) -> Result<bool, String>,
    ) -> Result<Option<u64>, String> {
        let mut files = Vec::new();
        let mut bytes = 0u64;
        for entry in
            fs::read_dir(&self.backups).map_err(|e| error("cannot read backup directory", e))?
        {
            let entry = entry.map_err(|e| error("cannot read backup entry", e))?;
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            let backup = name.strip_suffix(".bak").is_some_and(|hash| {
                hash.len() == 64 && hash.bytes().all(|byte| byte.is_ascii_hexdigit())
            });
            let temporary = name
                .strip_prefix(".patcher-")
                .and_then(|name| name.strip_suffix(".tmp"))
                .and_then(|name| name.split_once('-'))
                .is_some_and(|(pid, nonce)| {
                    !pid.is_empty()
                        && !nonce.is_empty()
                        && pid
                            .bytes()
                            .chain(nonce.bytes())
                            .all(|byte| byte.is_ascii_digit())
                });
            if !backup && !temporary && name != "state" {
                continue;
            }
            let path = entry.path();
            let metadata =
                fs::symlink_metadata(&path).map_err(|e| error("cannot inspect backup file", e))?;
            if !platform::regular(&metadata) {
                return Err(
                    "backup entry must be a regular file, symlinks are not supported".into(),
                );
            }
            bytes = bytes
                .checked_add(metadata.len())
                .ok_or("backup size overflow")?;
            files.push(path);
        }
        if files.is_empty() {
            return Ok(Some(0));
        }
        if !confirm(bytes)? {
            return Ok(None);
        }
        self.verify_path()?;
        for path in files {
            fs::remove_file(&path).map_err(|e| {
                error(
                    &format!("cleanup incomplete, cannot remove {}", path.display()),
                    e,
                )
            })?;
        }
        sync_dir(&self.backups)?;
        Ok(Some(bytes))
    }

    pub fn restore(&mut self) -> Result<bool, String> {
        let mut state = String::new();
        platform::options()
            .read(true)
            .open(self.backups.join("state"))
            .and_then(|file| file.take(256).read_to_string(&mut state))
            .map_err(|e| error("cannot read backup state", e))?;
        let hashes: Vec<_> = state.lines().collect();
        if hashes.len() != 2
            || hashes
                .iter()
                .any(|hash| hash.len() != 64 || !hash.bytes().all(|byte| byte.is_ascii_hexdigit()))
        {
            return Err("invalid backup state".into());
        }
        let current = digest(self.source.as_mut().ok_or("transaction is complete")?)?;
        if current == hashes[0] {
            return Ok(false);
        }
        if current != hashes[1] {
            return Err("executable changed since patching, backup retained".into());
        }
        let mut backup = platform::options()
            .read(true)
            .open(self.backups.join(format!("{}.bak", hashes[0])))
            .map_err(|e| error("cannot open backup", e))?;
        let metadata_source = backup
            .try_clone()
            .map_err(|e| error("cannot inspect backup", e))?;
        let staged = Staged::new(
            self.target.parent().unwrap(),
            &mut backup,
            Some(&metadata_source),
            hashes[0],
        )?;
        self.backup(&current)?;
        self.replace(&staged, &current)?;
        Ok(true)
    }
}
