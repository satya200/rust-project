fn main() {
    println!("cargo:rustc-link-search=native=/mnt/rust-dev/rust_dynlib_ctime/deviceutils-lib/target/debug");
    println!("cargo:rustc-link-lib=dylib=deviceutils_lib");
}

