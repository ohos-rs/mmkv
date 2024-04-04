use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let ndk = env::var("OHOS_NDK_HOME").unwrap();
    let source_code = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("MMKV")
        .join("Core");

    // use cmake to build mmkv
    let mmkv = Config::new(source_code)
        .define("CMAKE_TOOLCHAIN_FILE", format!("{}/native/build/cmake/ohos.toolchain.cmake",ndk))
        .define("OHOS_STL","c++_shared")
        .define("OHOS_ARCH","arm64-v8a")
        .define("OHOS_PLATFORM","OHOS")
        .build_target("core")
        .build();

    println!("cargo:rustc-link-search=native={}/build", mmkv.display());
    println!("cargo:rustc-link-search=native={}/build/core", mmkv.display());
    println!("cargo:rustc-link-lib=static=core");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I").clang_arg("")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
