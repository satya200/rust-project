use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::raw::c_char;
use std::ptr;

/// Expose this as a C-compatible API returning a raw pointer.
/// Make sure to free the returned pointer later using `CString::from_raw`.
#[unsafe(no_mangle)]
pub extern "C" fn get_device_prop(path: *const c_char, key: *const c_char) -> *mut c_char {
    // Step 1: Convert C strings to Rust strings
    let c_path = unsafe { CStr::from_ptr(path) }.to_str().ok();
    let c_key = unsafe { CStr::from_ptr(key) }.to_str().ok();

    // Early return null if conversion fails
    let (path_str, key_str) = match (c_path, c_key) {
        (Some(p), Some(k)) => (p, k),
        _ => return ptr::null_mut(),
    };

    // Step 2: Open file
    let file = match File::open(path_str) {
        Ok(f) => f,
        Err(_) => return ptr::null_mut(),
    };

    let reader = BufReader::new(file);

    // Step 3: Look for the key
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l.trim().to_string(),
            Err(_) => continue,
        };

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key_str {
                let result = v.trim().trim_matches('"').to_string();
                return match CString::new(result) {
                    Ok(s) => s.into_raw(),
                    Err(_) => ptr::null_mut(),
                };
            }
        }
    }

    // Step 4: If not found
    ptr::null_mut()
}

