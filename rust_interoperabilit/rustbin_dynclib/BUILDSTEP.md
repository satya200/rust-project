# Build deviceutils-lib:
  1. cd deviceutils-lib
  2. make

# Build device
  1. export LD_LIBRARY_PATH=/mnt/rust-dev/rustbin_dynclib/deviceutils-lib:$LD_LIBRARY_PATH
  2. cargo build
  2. cargo run

NOTE: For this use 2021 version in cargo.toml file

