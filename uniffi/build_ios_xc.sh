#!/bin/bash

FFI="mobile_ffi"
cd $FFI
XC_NAME="mobile"
# Add the iOS targets and build
for TARGET in \
        aarch64-apple-ios \
        aarch64-apple-ios-sim \
        x86_64-apple-ios
do
#   rustup target add $TARGET
    if [ $TARGET == "armv7s-apple-ios" ]; then
        
        cargo +nightly build -Z build-std=std --release --target=$TARGET --features=mock
    else
        cargo build --release --target=$TARGET
    fi

done
cd -



# Generate bindings
cargo run --bin uniffi-bindgen generate --library ./target/aarch64-apple-ios/release/lib${FFI}.dylib --language swift --out-dir ./bindings

# Rename *.modulemap to module.modulemap
mv ./bindings/${FFI}FFI.modulemap ./bindings/module.modulemap

# Move the Swift file to the project
rm ./dist/ios/swift/${FFI}.swift
mv ./bindings/${FFI}.swift ./dist/ios/swift/${FFI}.swift

mkdir sim
lipo -create ./target/x86_64-apple-ios/release/lib${FFI}.a ./target/aarch64-apple-ios-sim/release/lib${FFI}.a -output ./sim/lib${FFI}.a
# lipo -create ./target/aarch64-apple-ios/release/lib${FFI}.a ./target/armv7s-apple-ios/release/lib${FFI}.a -output ./sm/lib${FFI}.a

# Recreate XCFramework
rm -rf "dist/ios/$XC_NAME.xcframework"
xcodebuild -create-xcframework \
        -library ./sim/lib${FFI}.a -headers ./bindings \
        -library ./target/aarch64-apple-ios/release/lib${FFI}.a -headers ./bindings \
        -output "dist/ios/$XC_NAME.xcframework"

# Cleanup
rm -rf bindings sim ios
