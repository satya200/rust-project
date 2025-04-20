use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[unsafe(no_mangle)]
pub extern "C" fn get_device_prop(path: *const c_char, key: *const c_char) -> *mut c_char {
    let c_path = unsafe { CStr::from_ptr(path) }.to_str().ok();
    let c_key = unsafe { CStr::from_ptr(key) }.to_str().ok();

    if let (Some(path), Some(key)) = (c_path, c_key) {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);

            for line in reader.lines().flatten() {
                let line = line.trim();
                if line.starts_with('#') || line.is_empty() {
                    continue;
                }

                if let Some((k, v)) = line.split_once('=') {
                    if k.trim() == key {
                        let value = v.trim().trim_matches('"');
                        return CString::new(value).ok().map_or(std::ptr::null_mut(), |s| s.into_raw());
                    }
                }
            }
        }
    }

    std::ptr::null_mut()
}

