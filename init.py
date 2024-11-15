import argparse
import os
import shutil


def rename_directory(old_name, new_name):
    try:
        os.rename(old_name, new_name)
        print(f" rename '{old_name}' to '{new_name}'")
    except FileNotFoundError:
        print(f"err：dir '{old_name}' not exist")
    except FileExistsError:
        print(f"err：dir '{new_name}' exist")
    except Exception as e:
        print(f"err：{e}")


def replace_mobile_in_file(file_path, replacement_string):
    replace_str_in_file(file_path, "mobile" , replacement_string)

def replace_str_in_file(file_path, origin_string, replacement_string):
    try:
        # 读取文件内容
        with open(file_path, 'r', encoding='utf-8') as file:
            lines = file.readlines()

        # 替换每一行中的 'mobile'
        modified_lines = [line.replace(origin_string, replacement_string) for line in lines]

        # 将修改后的内容写回到原文件
        with open(file_path, 'w', encoding='utf-8') as file:
            file.writelines(modified_lines)

        print(f"file '{file_path}'  '{origin_string}' replace '{replacement_string}'")

    except FileNotFoundError:
        print(f"err: file '{file_path}' not exist")
    except Exception as e:
        print(f"err：{e}")

def delete_path(path):
    """删除文件或文件夹（包括非空文件夹）"""
    
    if os.path.isfile(path):
        try:
            os.remove(path)
            print(f"file {path} remove")
        except Exception as e:
            print(f"remove file {path} err: {e}")
    
    elif os.path.isdir(path):
        try:
            shutil.rmtree(path)
            print(f"dir {path} remove")
        except Exception as e:
            print(f"remove dir {path} err: {e}")
    
    else:
        print(f"{path} not exit")

def parse_args():
    # create parser
    parser = argparse.ArgumentParser(description="Mobile Rust Arg Parser")
    # add argument
    parser.add_argument('--lib_name', type=str, help='lib name', required=False)
    parser.add_argument('--android_ndk_home', type=str, help='Android ndk', required=False)
    parser.add_argument('--android_pkg_name', type=str, help='kotlin code pkg name', required=False)
    parser.add_argument('--ohos_ndk_home', type=str, help='Harmony os ndk', required=False)
    # parse args
    args = parser.parse_args()
    modify_rust_project(args)

def modify_rust_project(args):
    # modify lib name dir
    if args.lib_name:
        new_lib_dir = "./"  + args.lib_name
        rename_directory("./mobile", new_lib_dir)
        replace_mobile_in_file(new_lib_dir + "/Cargo.toml", args.lib_name)
        replace_mobile_in_file(new_lib_dir + "/bin/main.rs", args.lib_name)


    # modify lib ffi dir
    if args.lib_name:
        new_ffi_dir = "./uniffi/{}_ffi".format(args.lib_name)
        rename_directory("./uniffi/mobile_ffi", new_ffi_dir)
        replace_mobile_in_file(new_ffi_dir + "/Cargo.toml", args.lib_name)
        replace_mobile_in_file(new_ffi_dir + "/src/lib.rs", args.lib_name)
        replace_mobile_in_file("uniffi/build_android.sh", args.lib_name)
        replace_mobile_in_file("uniffi/build_ios_xc.sh", args.lib_name)
        replace_mobile_in_file("uniffi/Cargo.toml", args.lib_name)
    if args.android_ndk_home:
        replace_str_in_file("uniffi/build_android.sh", "to_strip_so", args.android_ndk_home)
    if args.android_pkg_name:
        replace_str_in_file(new_ffi_dir + "/uniffi.toml", "com.example.mobile.ffi" , args.android_pkg_name)


    # modify ohos
    if args.lib_name:
        replace_mobile_in_file("./ohos/Cargo.toml", args.lib_name)
        replace_mobile_in_file("./ohos/src/lib.rs", args.lib_name)
        replace_mobile_in_file("./ohos/build_ohos.sh", args.lib_name)
    if args.ohos_ndk_home:
        replace_str_in_file("./ohos/build_ohos.sh", "ohos_ndk", args.ohos_ndk_home)

    # delete old build
    delete_path("./uniffi/dist/android/kotlin/")
    delete_path("./uniffi/dist/android/strip/arm64-v8a/libmobile_ffi.so")
    delete_path("./uniffi/dist/android/strip/armeabi-v7a/libmobile_ffi.so")
    delete_path("./uniffi/dist/android/symbol/arm64-v8a/libmobile_ffi.so")
    delete_path("./uniffi/dist/android/symbol/armeabi-v7a/libmobile_ffi.so")
    delete_path("./uniffi/dist/ios/swift/mobile_ffi.swift")
    delete_path("./uniffi/dist/ios/mobile.xcframework/")


if __name__ == '__main__':
    parse_args()
