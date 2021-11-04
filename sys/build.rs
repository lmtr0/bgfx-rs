use std::{env::current_dir, process::Command};

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
    let mut cmd = Command::new(format!("{}/bx/tools/bin/{}/genie", curdir, os));
    cmd.current_dir(format!("{}/bgfx", curdir));
    // cmd.arg("--with-shared-lib");

    if os == "windows" {
        cmd.args(["--gcc=mingw-gcc", "gmake"]);
        makefile_target = "gmake-mingw-gcc";
    }
    else if os == "darwing" {
        cmd.args(["--gcc=osx-x64", "gmake"]);
        makefile_target = "gmake-osx-x64";
    }
    else if os == "linux" {
        cmd.args(["--gcc=linux-gcc", "gmake"]);
        makefile_target = "gmake-linux";
    }
    else {
        panic!("Target not supported");
    }

    cmd.spawn().expect("Failed to start generate command").wait_with_output().expect("Failed to execute the generate command");

    //? build
    let mut cmd = Command::new("make");
    cmd.args(["-R", "-C", format!("bgfx/.build/projects/{}", makefile_target).as_str(), "config=release64"]);
    cmd.spawn().expect("Failed to build bgfx project").wait_with_output().expect("Failed to execute the make command to build the bgfx project");

    if env.contains("windows") {
        // todo fixme
    } else if env.contains("darwin") {
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
    } else {
        // bgfx libs
        println!("cargo:rustc-link-search={}/bgfx/.build/linux64_gcc/bin", curdir);
        println!("cargo:rustc-link-lib=bgfxRelease");
        println!("cargo:rustc-link-lib=bimg_decodeRelease");
        println!("cargo:rustc-link-lib=bimgRelease");
        println!("cargo:rustc-link-lib=bxRelease");

        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=X11");

        // let mut build = cc::Build::new();
        // let libdir = format!("{}/bgfx/.build/linux64_gcc/bin", curdir);
        // build.file(format!("{}/libbgfxRelease.a", libdir));
        // build.file(format!("{}/libbimg_decodeRelease.a", libdir));
        // build.file(format!("{}/libbimgRelease.a", libdir));
        // build.file(format!("{}/libbxRelease.a", libdir));
        // build.compile("bgfx_sys");
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

    println!("cargo:warning=Finished building");
}
