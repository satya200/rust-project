use std::env;
use std::path::PathBuf;
//use std::thread::Builder;

fn main() {
    println!("cargo:rerun-if-changed=/mnt/rust-dev/rustbin_dynclib/deviceutils-lib/deviceutils.h");

    let bindings = bindgen::Builder::default()
    //let bindings = Builder::default()
        .header("/mnt/rust-dev/rustbin_dynclib/deviceutils-lib/deviceutils.h")
        .generate_inline_functions(true)
        .layout_tests(false)
        .generate_comments(true)
        .clang_arg("-I../deviceutils-lib") // optional if headers include other files
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    // Link the shared library
    println!("cargo:rustc-link-search=native=/mnt/rust-dev/rustbin_dynclib/deviceutils-lib");
    println!("cargo:rustc-link-lib=dylib=deviceutils-lib");
}

