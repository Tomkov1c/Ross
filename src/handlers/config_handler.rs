use std::path::PathBuf;

pub fn get_or_create_config_dir() -> PathBuf {
    let dir = dirs::config_dir().unwrap().join("ross");
    std::fs::create_dir_all(&dir).unwrap();

    #[cfg(target_os = "windows")]
    set_hidden(&dir);

    dir
}

pub fn create_scheme_dir() -> PathBuf {
    let dir = get_or_create_config_dir().join("schemes");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn get_config_path(file_name: &str) -> PathBuf {
    get_or_create_config_dir().join(file_name)
}

#[cfg(target_os = "windows")]
fn set_hidden(path: &std::path::Path) {
    use std::os::windows::ffi::OsStrExt;

    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();

    unsafe {
        let attrs = windows_sys::Win32::Storage::FileSystem::GetFileAttributesW(wide.as_ptr());
        if attrs != u32::MAX {
            windows_sys::Win32::Storage::FileSystem::SetFileAttributesW(
                wide.as_ptr(),
                attrs | windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN,
            );
        }
    }
}