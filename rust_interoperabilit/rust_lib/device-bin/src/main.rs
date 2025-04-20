use deviceutils_lib::get_device_prop;

fn main() {
    let prop_file = "/mnt/rust-dev/rust_lib/device.prop";
    let key = "device_name";

    match get_device_prop(prop_file, key) {
        Some(value) => println!("{key} = {value}"),
        None => eprintln!("Property `{key}` not found."),
    }
}

