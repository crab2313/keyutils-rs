use std::ffi::CStr;
use std::os::raw::c_char;

use keyutils_sys::*;

unsafe fn static_string(str: &[c_char; 0]) -> String {
    let cstr = CStr::from_ptr(str as *const c_char);
    let cstr = cstr.to_str().unwrap();
    cstr.to_string()
}

pub fn version() -> String {
    unsafe { static_string(&keyutils_version_string) }
}

pub fn build() -> String {
    unsafe { static_string(&keyutils_build_string) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_string() {
        println!("build date {}", build());
        assert_eq!(version(), "keyutils-1.6.3".to_string());
    }
}
