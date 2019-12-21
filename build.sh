#!/bin/sh

# Build and create header
cargo build

# Display ferry
cargo run --example test_scale

# Group binary with lipo
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
# Print NonFat -> Ok
lipo -info ./target/aarch64-apple-ios/debug/libscale.a
lipo -info ./target/x86_64-apple-ios/debug/libscale.a
# Group in one
lipo -create ./target/aarch64-apple-ios/debug/libscale.a ./target/x86_64-apple-ios/debug/libscale.a -output ./target/libscale.a
# Print architecture
lipo -info ./target/libscale.a
