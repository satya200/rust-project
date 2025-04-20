//extern crate deviceutilslib;

use deviceutils_lib::get_device_prop;
use std::env;

fn main() {
    let prop_file = "/mnt/rust-dev/rust_lib/device.prop";
    let key = "device_name";

    match unsafe { deviceutils_lib::get_device_prop(prop_file, key) } {
        Some(value) => println!("{key} = {value}"),
        None => eprintln!("Property `{key}` not found."),
    }
}

