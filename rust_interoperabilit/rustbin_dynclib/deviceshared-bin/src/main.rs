#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

/*mod bindings {
    #![allow(dead_code)]
    #![allow(improper_ctypes)]

    // Include bindgen output inside module (not inside unsafe)
    //include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    pub unsafe mod raw {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }
}*/

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//use bindings::*;
use std::ffi::{CString, CStr};

fn main() {
    let path = CString::new("../deviceutils-lib/device.prop").unwrap();
    let key = CString::new("device_name").unwrap();

    unsafe {
        let prop = get_device_prop_struct(path.as_ptr(), key.as_ptr());

        if prop.status == Status_STATUS_OK {
            let key_str = CStr::from_ptr(prop.key).to_string_lossy();
            let val_str = CStr::from_ptr(prop.value).to_string_lossy();
            println!("{} = {}", key_str, val_str);
        } else {
            eprintln!("Property not found.");
        }
    }
}

