# Link Dynamic lib at compile time

# Build deviceutils-lib
    1. cd deviceutils-lib
    2. cargo build
# Build deviceshared-bin
    1. cd deviceshared-bin
    2. Set env export LD_LIBRARY_PATH=/mnt/rust-dev/rust_dynlib_ctime/deviceutils-lib/target/debug:$LD_LIBRARY_PATH
    3. cargo build
    4. cargo run

# Link RUST dynamic lib with C binary

Below is the command and output

gcc mytest.c -L/mnt/rust-dev/rust_dynlib_ctime/deviceutils-lib/target/debug/ -ldeviceutils_lib -o mytest
root@91fa0a8dd493:/mnt/rust-dev/rust_dynlib_ctime# ./mytest 
device_name = xglobal

