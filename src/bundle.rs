use std::{
    fs::File,
    io::Read,
    path::{Component, Path, PathBuf},
};

pub fn executable(path: &Path) -> Result<PathBuf, String> {
    if !path.is_dir() {
        return Ok(path.to_path_buf());
    }
    if path.extension().is_some_and(|extension| extension == "app") {
        return app_executable(path);
    }
    let app = path.join("DaVinci Resolve.app");
    if app.is_dir() {
        return app_executable(&app);
    }
    ["bin/resolve", "Resolve.exe"]
        .iter()
        .map(|name| path.join(name))
        .find(|executable| executable.is_file())
        .ok_or_else(|| "resolve executable not found in folder".into())
}

fn app_executable(path: &Path) -> Result<PathBuf, String> {
    let mut data = Vec::new();
    File::open(path.join("Contents/Info.plist"))
        .and_then(|file| file.take(1024 * 1024 + 1).read_to_end(&mut data))
        .map_err(|e| format!("cannot read app info: {e}"))?;
    if data.len() > 1024 * 1024 {
        return Err("app info is too large".into());
    }
    let info = plist::Value::from_reader(std::io::Cursor::new(data))
        .map_err(|e| format!("invalid app info: {e}"))?;
    let name = info
        .as_dictionary()
        .and_then(|dict| dict.get("CFBundleExecutable"))
        .and_then(plist::Value::as_string)
        .ok_or("app executable name not found")?;
    let mut components = Path::new(name).components();
    if !matches!(components.next(), Some(Component::Normal(_)))
        || components.next().is_some()
        || name.contains(['/', '\\', '\0'])
    {
        return Err("app executable name must be a single filename".into());
    }
    Ok(path.join("Contents/MacOS").join(name))
}

pub fn containing_app(executable: &Path) -> Option<&Path> {
    let macos = executable.parent()?;
    let contents = macos.parent()?;
    let app = contents.parent()?;
    (macos.file_name()? == "MacOS"
        && contents.file_name()? == "Contents"
        && app.extension()? == "app")
        .then_some(app)
}
