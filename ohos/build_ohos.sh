#cargo clean
FFI="mobile_oh"
OHOS_NDK="ohos_ndk"
OHOS_NDK_HOME=${OHOS_NDK} ohrs build --dist=./dist/symbol/ --release
# copy symbol so for addr2line

cd dist
rm -rf strip
mkdir strip
cp -r symbol/* ./strip/
cd -

# strip so debuginfo
${OHOS_NDK}/native/llvm/bin/llvm-strip --strip-all ./dist/strip/arm64-v8a/lib${FFI}.so
${OHOS_NDK}/native/llvm/bin/llvm-strip --strip-all ./dist/strip/armeabi-v7a/lib${FFI}.so
${OHOS_NDK}/native/llvm/bin/llvm-strip --strip-all ./dist/strip/x86_64/lib${FFI}.so

