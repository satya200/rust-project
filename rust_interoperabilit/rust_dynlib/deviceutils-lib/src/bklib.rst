use std::ffi::{CStr, CString};
use std::os::raw::{c_char};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[unsafe(no_mangle)]
pub extern "C" fn get_device_prop(path: *const c_char, key: *const c_char) -> *mut c_char {
    let c_path = unsafe { CStr::from_ptr(path) }.to_str().ok()?;
    let c_key = unsafe { CStr::from_ptr(key) }.to_str().ok()?;

    let file = File::open(c_path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == c_key {
                let result = v.trim().trim_matches('"');
                return CString::new(result).ok()?.into_raw();
            }
        }
    }

    std::ptr::null_mut()
}

