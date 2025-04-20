use libloading::{Library, Symbol};
use std::ffi::{CString, CStr};

type GetDeviceProp = unsafe extern "C" fn(*const u8, *const u8) -> *mut u8;

fn main() {
    unsafe {
    let lib = Library::new("/mnt/rust-dev/rust_dynlib/deviceutils-lib/target/debug/libdeviceutils_lib.so")
        .expect("Failed to load library");

        let func: Symbol<GetDeviceProp> = lib.get(b"get_device_prop")
            .expect("Failed to load symbol");

        let path = CString::new("/Users/ssahu777/rust_practice/rust-project/project/hello/rust_dynlib/deviceshared-bin/device.prop").unwrap();
        let key = CString::new("device_name").unwrap();

        let result_ptr = func(path.as_ptr(), key.as_ptr());

        if !result_ptr.is_null() {
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            println!("device_name = {}", result);
        } else {
            eprintln!("Property not found.");
        }
    }
}

