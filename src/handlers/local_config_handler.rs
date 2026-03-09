use std::env;
use std::fs;
use std::path::PathBuf;

pub fn search_dir_for_local_config(path: &std::path::Path) -> bool {
    let ross_dir = path.join(".ross");

    ross_dir.exists()
}

pub fn search_env_dir_for_local_config() -> bool {
    let current_dir = env::current_dir().expect("");

    search_dir_for_local_config(&current_dir)
}

pub fn create_local_config_at_env() -> Option<PathBuf> {
    if !search_env_dir_for_local_config() {
        let current_dir = env::current_dir().expect("");
        let ross_dir = current_dir.join(".ross");
        fs::create_dir(&ross_dir).expect("");

        #[cfg(target_os = "windows")]
        set_dir_as_hidden(&ross_dir);

        Some(ross_dir)
    }else {
        None
    }
}

#[cfg(target_os = "windows")]
fn set_dir_as_hidden(path: &std::path::Path) {
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