use crate::transaction::error;
use std::{
    fs::{self, File, Metadata, OpenOptions},
    io,
    mem::{size_of, zeroed},
    os::windows::{
        ffi::OsStrExt,
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawHandle,
    },
    path::Path,
    ptr::null_mut,
};
use windows_sys::Win32::{
    Foundation::{
        CloseHandle, ERROR_LOCK_VIOLATION, ERROR_NO_MORE_FILES, ERROR_SHARING_VIOLATION,
        INVALID_HANDLE_VALUE,
    },
    Security::{
        DACL_SECURITY_INFORMATION, GROUP_SECURITY_INFORMATION, GetKernelObjectSecurity,
        GetSecurityDescriptorControl, OWNER_SECURITY_INFORMATION,
        PROTECTED_DACL_SECURITY_INFORMATION, SE_DACL_PROTECTED, SetKernelObjectSecurity,
        UNPROTECTED_DACL_SECURITY_INFORMATION,
    },
    Storage::FileSystem::{
        BY_HANDLE_FILE_INFORMATION, FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_OPEN_REPARSE_POINT,
        FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_DELETE, FILE_SHARE_READ,
        GetFileInformationByHandle, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
        WRITE_DAC, WRITE_OWNER,
    },
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW,
        TH32CS_SNAPPROCESS,
    },
};

pub fn options() -> OpenOptions {
    let mut options = OpenOptions::new();
    options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    options
}

pub fn temporary_options() -> OpenOptions {
    let mut options = options();
    options.access_mode(FILE_GENERIC_READ | FILE_GENERIC_WRITE | WRITE_DAC | WRITE_OWNER);
    options
}

pub fn open_source(path: &Path) -> io::Result<File> {
    options()
        .read(true)
        .write(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_DELETE)
        .open(path)
}

pub fn create_dir(path: &Path) -> io::Result<()> {
    fs::create_dir(path)
}
pub fn regular(metadata: &Metadata) -> bool {
    metadata.is_file() && metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT == 0
}
pub fn directory(metadata: &Metadata) -> bool {
    metadata.is_dir() && metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT == 0
}
pub fn busy(error: &io::Error) -> bool {
    matches!(
        error.raw_os_error().map(|code| code as u32),
        Some(ERROR_SHARING_VIOLATION | ERROR_LOCK_VIOLATION)
    )
}

fn identity(file: &File) -> io::Result<(u32, u32, u32)> {
    let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { zeroed() };
    if unsafe { GetFileInformationByHandle(file.as_raw_handle(), &mut info) } == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok((
        info.dwVolumeSerialNumber,
        info.nFileIndexHigh,
        info.nFileIndexLow,
    ))
}

pub fn same_file(source: &File, path: &Path) -> io::Result<bool> {
    let current = options().read(true).open(path)?;
    Ok(regular(&current.metadata()?) && identity(source)? == identity(&current)?)
}

pub fn preserve(file: &File, source: &File) -> io::Result<()> {
    let flags = OWNER_SECURITY_INFORMATION | GROUP_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION;
    let mut size = 0;
    unsafe {
        GetKernelObjectSecurity(source.as_raw_handle(), flags, null_mut(), 0, &mut size);
    }
    if size == 0 {
        return Err(io::Error::last_os_error());
    }
    let mut descriptor = vec![0u8; size as usize];
    if unsafe {
        GetKernelObjectSecurity(
            source.as_raw_handle(),
            flags,
            descriptor.as_mut_ptr().cast(),
            size,
            &mut size,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    let mut control = 0;
    let mut revision = 0;
    if unsafe {
        GetSecurityDescriptorControl(descriptor.as_mut_ptr().cast(), &mut control, &mut revision)
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    let flags = flags
        | if control & SE_DACL_PROTECTED != 0 {
            PROTECTED_DACL_SECURITY_INFORMATION
        } else {
            UNPROTECTED_DACL_SECURITY_INFORMATION
        };
    if unsafe {
        SetKernelObjectSecurity(
            file.as_raw_handle(),
            flags,
            descriptor.as_ptr().cast_mut().cast(),
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    file.set_permissions(source.metadata()?.permissions())
}

pub fn sync_dir(_: &Path) -> io::Result<()> {
    Ok(())
}

pub fn replace(source: &Path, target: &Path) -> io::Result<()> {
    let source: Vec<_> = source.as_os_str().encode_wide().chain(Some(0)).collect();
    let target: Vec<_> = target.as_os_str().encode_wide().chain(Some(0)).collect();
    if unsafe {
        MoveFileExW(
            source.as_ptr(),
            target.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

pub fn ensure_idle(_: &Path) -> Result<(), String> {
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot == INVALID_HANDLE_VALUE {
        return Err(error(
            "cannot check running processes",
            io::Error::last_os_error(),
        ));
    }
    let result = (|| {
        let mut entry: PROCESSENTRY32W = unsafe { zeroed() };
        entry.dwSize = size_of::<PROCESSENTRY32W>() as u32;
        let mut available = unsafe { Process32FirstW(snapshot, &mut entry) };
        while available != 0 {
            let end = entry
                .szExeFile
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(entry.szExeFile.len());
            let name = String::from_utf16_lossy(&entry.szExeFile[..end]);
            if name.eq_ignore_ascii_case("resolve.exe")
                || name.eq_ignore_ascii_case("resolve.patched.exe")
            {
                return Err(format!(
                    "resolve is running (pid {}), close it and retry",
                    entry.th32ProcessID
                ));
            }
            available = unsafe { Process32NextW(snapshot, &mut entry) };
        }
        let failure = io::Error::last_os_error();
        if failure.raw_os_error() != Some(ERROR_NO_MORE_FILES as i32) {
            return Err(error("cannot enumerate processes", failure));
        }
        Ok(())
    })();
    unsafe {
        CloseHandle(snapshot);
    }
    result
}
