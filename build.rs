use cc::Build;
use std::collections::HashMap;
use std::{env, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

fn main() {
    let h = env::current_dir().unwrap().join("mmkv/Core");

    let is_release = env::var("PROFILE")
        .unwrap_or(String::from("DEBUG"))
        .eq("release");

    println!("cargo:rustc-link-search={:?}", &h);
    println!("cargo:rustc-link-lib=static=clang_rt.builtins");
    println!("cargo:rerun-if-changed=wrapper.cpp");

    let v = env::var("TARGET").unwrap();

    let target = HashMap::from([
        (
            "aarch64-unknown-linux-ohos",
            ("aarch64-linux-ohos/c++", "aarch64-linux-ohos"),
        ),
        (
            "armv7-unknown-linux-ohos",
            ("arm-linux-ohos/c++/a7_softfp_neon-vfpv4", "arm-linux-ohos"),
        ),
        (
            "x86_64-unknown-linux-ohos",
            ("x86_64-linux-ohos/c++", "x86_64-linux-ohos"),
        ),
    ]);

    let t = target.get(&v.as_str()).unwrap();

    fn is_cpp_file(entry: &DirEntry) -> bool {
        entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map_or(false, |e| e == "cpp" || e == "S")
    }

    let cpp_files: Vec<PathBuf> = WalkDir::new(&h)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_cpp_file)
        .map(|i| i.into_path())
        .collect();

    let mut builder = Build::new();

    if v.as_str() == "aarch64-unknown-linux-ohos" {
        builder
            .asm_flag("-x")
            .asm_flag("assembler-with-cpp")
            .asm_flag("-march=armv8-a+crypto");
    }

    builder
        .define("MMKV_EMBED_ZLIB", "1")
        .file("wrapper.cpp")
        .files(cpp_files)
        .debug(!is_release)
        .cpp(true)
        .include(&h)
        .cpp_link_stdlib("c++_static")
        .compile("native_mmkv");

    napi_build_ohos::setup();
}
