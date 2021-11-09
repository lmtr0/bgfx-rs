use std::{env::{current_dir}, path::Path, process::Command};

fn link_bgfx(searchpath: String) {
    println!("cargo:rustc-link-search={}", searchpath);
    println!("cargo:rustc-link-lib=bgfxRelease");
    println!("cargo:rustc-link-lib=bimg_decodeRelease");
    println!("cargo:rustc-link-lib=bimgRelease");
    println!("cargo:rustc-link-lib=bxRelease");
}


fn main() {
    
    println!("cargo:warning=You are building bgfx, bimg and bx from the source code, this may take a while if it's the first time");

    let env = std::env::var("TARGET").unwrap();

    let isdarwin = env.contains("darwin");
    let iswindows = env.contains("windows");
    let isunix = env.contains("linux");


    let curdir = current_dir().unwrap().as_path().display().to_string();
    
    //? generate ffi
    if isunix {
        let bindings = bindgen::builder()
            .layout_tests(false)
            .prepend_enum_name(false)
            .allowlist_function("bgfx.*")
            .allowlist_type("bgfx.*")
            .allowlist_type("BGFX.*")
            .allowlist_var("bgfx.*")
            .allowlist_var("BGFX.*")
            // .allowlist_var("__va_list_tag")
            // .blocklist_item("int_.*")
            // .blocklist_item("uint_.*")
            // .blocklist_item("__gnu_va_list")
            // // .blocklist_item("__.*")
            // .blocklist_item("intmax_t")
            // .blocklist_item("uintmax_t")
            // .blocklist_item("wchar_t")
            // .blocklist_item("_Float32")
            // .blocklist_item("_Float64")
            // .blocklist_item("_Float32x")
            // .blocklist_item("_Float64x")
            
            .header("src/header.h")
            .allowlist_recursively(true)
            .clang_arg("-Ibx/include")
            .generate().expect("Failed to generate bindings");

        bindings
            .write_to_file("src/ffi.rs")
            .expect("Couldn't write bindings!");
    }

    
    //? Generate
    // defaults to /opt/osxcross
    if std::env::var("OSXCROSS").is_err() {
        std::env::set_var("OSXCROSS", "/opt/osx")
    }

    let makefile_target;
    // Copy toolchain.lua file to bx/scripts/toolchain.lua
    if !Path::new("bgfx").exists() {
        Command::new("sh").arg(format!("{}/update.sh", &curdir)).current_dir(&curdir).spawn().expect("Failed to update sources").wait().expect("Failed to copy toolchain");
    }

    // copy the newers toolchain to the build direcoty
    Command::new("cp")
        .arg("toolchain.lua")
        .arg("bx/scripts/toolchain.lua")
        .spawn().expect("Failed to copy toolchain file to directory")
        .wait().expect("Failed to copy toolchain");

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

    if iswindows {
        cmd.args(["--gcc=mingw-gcc", "gmake"]);
        makefile_target = "gmake-mingw-gcc";
    }
    else if isdarwin {
        cmd.args(["--gcc=osx-x64", "gmake"]);
        makefile_target = "gmake-osx-x64";
    }
    else if isunix {
        cmd.args(["--gcc=linux-gcc", "gmake"]);
        // makefile_target = "gmake-linux-clang";
        makefile_target = "gmake-linux";
    }
    else {
        panic!("Target not supported");
    }

    println!("compiling makefile");
    cmd.spawn().expect("Failed to start generate command").wait_with_output().expect("Failed to execute the generate command");
    println!("Finished makefile");

    //? build
    let mut cmd = Command::new("make");
    cmd.args([
        "-R", 
        "-C", format!("bgfx/.build/projects/{}", makefile_target).as_str(), 
        "config=release64"
    ]);
    cmd.spawn().expect("Failed to build bgfx project").wait().expect("Failed to execute the make command to build the bgfx project");

    // bgfx libs
    if iswindows {
        println!("cargo:warning=Compiling to Windows");
        link_bgfx(format!("{}/bgfx/.build/win64_mingw-gcc/bin", curdir));

        println!("cargo:rustc-link-lib=winpthread");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=psapi");        
    } 
    else if isdarwin {
        println!("cargo:warning=Compiling to Darwin");
        link_bgfx(format!("{}/bgfx/.build/osx-x64/bin", curdir));  

        println!("cargo:rustc-link-lib=c++");
        
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=Metal");
        // println!("cargo:rustc-link-lib=framework=MetalKit");
    } 
    else if isunix {
        println!("cargo:warning=Compiling to Unix (linux)");
        link_bgfx(format!("{}/bgfx/.build/linux64_gcc/bin", curdir));  
        
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=X11");
    }
    else {
        panic!("OS not suported")
    }
    
    println!("cargo:warning=All Done");
}
