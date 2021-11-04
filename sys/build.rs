use std::{env::{current_dir}, path::Path, process::Command};

fn main() {
    println!("cargo:warning=You are building bgfx, bimg and bx from the source code, this may take a while if it's the first time");

    let curdir = current_dir().unwrap().as_path().display().to_string();
    
    let env = std::env::var("TARGET").unwrap();
    let os = match env.as_str() {
        "x86_64-apple-darwin" => "darwing",
        "x86_64-pc-windows-gnu" | "x86_64-uwp-windows-gnu" => "windows",
        "x86_64-unknown-linux-gnu" => "linux",
        _ => panic!("Environment not supported"),
    };
    //? Generate
    let makefile_target;
    // Copy toolchain.lua file to bx/scripts/toolchain.lua
    if !Path::new("bgfx").exists() {
        Command::new("sh").arg(format!("{}/update.sh", &curdir)).current_dir(&curdir).spawn().expect("Failed to update sources");
    }

    Command::new("cp")
        .arg("toolchain.lua")
        .arg("bx/scripts/toolchain.lua")
        .spawn().expect("Failed to copy toolchain file to directory");

    let mut cmd;
    if cfg!(windows) {
        cmd = Command::new(format!("{}/bx/tools/bin/windows/genie.exe", &curdir));
    }
    else if cfg!(macos) {
        cmd =  Command::new(format!("{}/bx/tools/bin/darwing/genie", &curdir));
    }
    else if cfg!(unix) {
        cmd =  Command::new(format!("{}/bx/tools/bin/linux/genie", &curdir));
    }
    else {
        panic!("Platform not supported for building");
    }

    cmd.current_dir(format!("{}/bgfx", &curdir));
    // cmd.arg("--with-shared-lib");

    if os == "windows" {
        cmd.args(["--gcc=mingw-gcc", "gmake"]);
        makefile_target = "gmake-mingw-gcc";
    }
    else if os == "darwing" {
        cmd.args(["--gcc=osx-x64", "gmake"]);
        makefile_target = "osx-x64";
    }
    else if os == "linux" {
        cmd.args(["--gcc=linux-gcc", "gmake"]);
        // makefile_target = "gmake-linux-clang";
        makefile_target = "gmake-linux";
    }
    else {
        panic!("Target not supported");
    }

    cmd.spawn().expect("Failed to start generate command").wait_with_output().expect("Failed to execute the generate command");

    //? build
    let mut cmd = Command::new("make");
    cmd.args([
        "-R", 
        "-C", format!("bgfx/.build/projects/{}", makefile_target).as_str(), 
        "config=release64"
    ]);
    cmd.spawn().expect("Failed to build bgfx project").wait_with_output().expect("Failed to execute the make command to build the bgfx project");

    // bgfx libs


    if env.contains("windows") {
        println!("cargo:rustc-link-search={}/bgfx/.build/win64_mingw-gcc/bin", curdir);
        println!("cargo:rustc-link-lib=bgfxRelease");
        println!("cargo:rustc-link-lib=bimg_decodeRelease");
        println!("cargo:rustc-link-lib=bimgRelease");
        println!("cargo:rustc-link-lib=bxRelease");

    } else if env.contains("darwin") {
        println!("cargo:rustc-link-search={}/bgfx/.build/osx_x64/bin", curdir);  
        println!("cargo:rustc-link-lib=bgfxRelease");
        println!("cargo:rustc-link-lib=bimg_decodeRelease");
        println!("cargo:rustc-link-lib=bimgRelease");
        println!("cargo:rustc-link-lib=bxRelease");

        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
    } else {
        println!("cargo:rustc-link-search={}/bgfx/.build/linux64_gcc/bin", curdir);  
        println!("cargo:rustc-link-lib=bgfxRelease");
        println!("cargo:rustc-link-lib=bimg_decodeRelease");
        println!("cargo:rustc-link-lib=bimgRelease");
        println!("cargo:rustc-link-lib=bxRelease");
        
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=vulkan");
    }


    //? generate ffi
    let bindings = bindgen::builder()
        .layout_tests(false)
        .prepend_enum_name(false)
        .allowlist_function("bgfx.*")
        .allowlist_type("bgfx_.*")
        .allowlist_type("BGFX_.*")
        .allowlist_var("bgfx_.*")
        .allowlist_var("BGFX_.*")
        .allowlist_var("__va_list_tag")
        .blocklist_item("int_.*")
        .blocklist_item("uint_.*")
        .blocklist_item("__gnu_va_list")
        // .blocklist_item("__.*")
        .blocklist_item("intmax_t")
        .blocklist_item("uintmax_t")
        .blocklist_item("wchar_t")
        .blocklist_item("_Float32")
        .blocklist_item("_Float64")
        .blocklist_item("_Float32x")
        .blocklist_item("_Float64x")
        
        .header("src/header.h")
        .allowlist_recursively(true)
        .clang_arg("-Ibx/include")
        .generate().expect("Failed to generate bindings");

    bindings
        .write_to_file("src/ffi.rs")
        .expect("Couldn't write bindings!");
}
