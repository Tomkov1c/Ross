
    // #[cfg(target_os = "windows")]
    // set_hidden(&dir);





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