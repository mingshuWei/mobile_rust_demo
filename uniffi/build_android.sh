FFI="mobile_ffi"
ANDROID_NDK="to_strip_so"
ANDROID_STRIP="${ANDROID_NDK}/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-strip"


cd $FFI
# cargo clean
cargo ndk build -r
cd ..
cargo run --bin uniffi-bindgen generate --library ./target/aarch64-linux-android/release/lib$FFI.so --language kotlin --out-dir ./bindings
cp -r bindings/com ./dist/android/kotlin
# cd bindings
# rm -rf com
# cd -

cp ./target/aarch64-linux-android/release/lib$FFI.so ./dist/android/symbol/arm64-v8a/
cp ./target/armv7-linux-androideabi/release/lib$FFI.so ./dist/android/symbol/armeabi-v7a/

mv ./target/aarch64-linux-android/release/lib$FFI.so ./dist/android/strip/arm64-v8a/
${ANDROID_STRIP} --strip-all ./dist/android/strip/arm64-v8a/lib$FFI.so
mv ./target/armv7-linux-androideabi/release/lib$FFI.so ./dist/android/strip/armeabi-v7a/
${ANDROID_STRIP} --strip-all ./dist/android/strip/armeabi-v7a/lib$FFI.so


