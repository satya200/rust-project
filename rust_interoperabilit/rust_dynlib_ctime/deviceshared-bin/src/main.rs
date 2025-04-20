use std::ffi::{CString, CStr};

unsafe extern "C" {
    fn get_device_prop(path: *const u8, key: *const u8) -> *mut u8;
}

fn main() {
    let path = CString::new("/mnt/rust-dev/rust_dynlib_ctime/device.prop").unwrap();
    let key = CString::new("device_name").unwrap();

    let value_ptr = unsafe { get_device_prop(path.as_ptr(), key.as_ptr()) };

    if value_ptr.is_null() {
        eprintln!("Key not found");
    } else {
        let value = unsafe { CStr::from_ptr(value_ptr) }.to_string_lossy();
        println!("device_name = {}", value);
    }
}

