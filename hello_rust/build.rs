fn main() {
    // 指定库搜索路径
    println!("cargo:rustc-link-search=native=./libs");

    // 动态库链接指令
    println!("cargo:rustc-link-lib=dylib=awtk");

    // 跨平台动态库复制
    let lib_name = match std::env::consts::OS {
        "windows" => "awtk.dll",
        "linux" => "libawtk.so",
        "macos" => "libawtk.dylib",
        _ => return,
    };

    let lib_src = format!("./libs/{}", lib_name);
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let target_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    if let Err(e) = std::fs::copy(&lib_src, target_dir.join(lib_name)) {
        println!("cargo:warning=Failed to copy {}: {}", lib_name, e);
    }
}
