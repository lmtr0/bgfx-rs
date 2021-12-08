use std::{env::{current_dir}, path::Path, process::Command};

fn main() {
    
    println!("cargo:warning=You are building bgfx, bimg and bx from the source code, this may take a while if it's the first time");
    let mut build = cc::Build::new();
    let env = std::env::var("TARGET").unwrap();

    let isdarwin = env.contains("darwin");
    let iswindows = env.contains("windows");
    let isunix = env.contains("linux");


    let curdir = current_dir().unwrap().as_path().display().to_string();
    
    //? Generate
    if !Path::new("bgfx").exists() {
        Command::new("sh").arg(format!("{}/update.sh", &curdir)).current_dir(&curdir).spawn().expect("Failed to update sources").wait().expect("Failed to copy toolchain");
    }

    //? generate ffi
    let bindings = bindgen::builder()
        .layout_tests(false)
        .prepend_enum_name(false)
        .allowlist_function("bgfx.*")
        .allowlist_type("bgfx.*")
        .allowlist_type("BGFX.*")
        .allowlist_var("bgfx.*")
        .allowlist_var("BGFX.*")
        
        .header("src/header.h")
        .allowlist_recursively(true)
        .clang_arg("-Ibx/include")
        .generate().expect("Failed to generate bindings");

    bindings
        .write_to_file("src/ffi.rs")
        .expect("Couldn't write bindings!");


    // ! build
    // defines - Currently not supporting WebGPU, GNM and Vulkan
    // OS support:
    // Windows - DirectX and Vulkan
    // macOS - Metal
    // Posix - Vulkan, OpenGL
    // In the future it would be good to make this configurable instead

    build.define("BGFX_CONFIG_RENDERER_WEBGPU", "0");
    build.define("BGFX_CONFIG_RENDERER_GNM", "0");
    build.define("BIMG_DECODE_ASTC", "0");

    if iswindows {
        build.include("bx/include/compat/msvc");
        build.include("bgfx/3rdparty/dxsdk/include");

        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_DIRECT3D11", "1");
        build.define("_WIN32", None);
        build.define("_HAS_EXCEPTIONS", "0");
        build.define("_SCL_SECURE", "0");
        build.define("_SECURE_SCL", "0");
        build.define("__STDC_LIMIT_MACROS", None);
        build.define("__STDC_FORMAT_MACROS", None);
        build.define("__STDC_CONSTANT_MACROS", None);
        build.define("_CRT_SECURE_NO_WARNINGS", None);
        build.define("_CRT_SECURE_NO_DEPRECATE", None);
        build.warnings(false);
    } 
    else if isdarwin {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_METAL", "1");
        build.include("bx/include/compat/osx");
    }
    else if isunix {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_OPENGL", "1");
    }

    build.define("BX_CONFIG_DEBUG", Some("0"));

    
    // bx
    build.include("bx/include");
    build.include("bx/3rdparty");
    
    // build.file("bx/src/allocator.cpp");
    build.file("bx/src/amalgamated.cpp");
    build.file("bx/src/bounds.cpp");
    build.file("bx/src/commandline.cpp");
    build.file("bx/src/crtnone.cpp");
    build.file("bx/src/debug.cpp");
    build.file("bx/src/dtoa.cpp");
    build.file("bx/src/easing.cpp");
    build.file("bx/src/file.cpp");
    build.file("bx/src/filepath.cpp");
    build.file("bx/src/hash.cpp");
    build.file("bx/src/math.cpp");
    build.file("bx/src/mutex.cpp");
    build.file("bx/src/os.cpp");
    build.file("bx/src/process.cpp");
    build.file("bx/src/semaphore.cpp");
    build.file("bx/src/settings.cpp");
    build.file("bx/src/sort.cpp");
    build.file("bx/src/string.cpp");
    build.file("bx/src/thread.cpp");
    build.file("bx/src/timer.cpp");
    build.file("bx/src/url.cpp");
    build.file("bx/src/bx.cpp");

    // bimg
    build.include("bimg/include");
    build.include("bimg/3rdparty");
    build.include("bimg/3rdparty/iqa/include/");
    build.include("bimg/3rdparty/astc-codec/include");
    build.include("bimg/3rdparty/tinyexr/deps/miniz/");
    
    build.file("bimg/src/image.cpp");
    build.file("bimg/src/image_cubemap_filter.cpp");
    build.file("bimg/src/image_decode.cpp");
    build.file("bimg/src/image_encode.cpp");
    build.file("bimg/src/image_gnf.cpp");
    
    // bgfx
    build.include("bgfx/include");
    build.include("bgfx/3rdparty");
    build.include("bgfx/3rdparty/khronos/");
    
    build.file("bgfx/src/amalgamated.cpp");
    build.file("bgfx/src/bgfx.cpp");
    build.file("bgfx/src/debug_renderdoc.cpp");
    build.file("bgfx/src/dxgi.cpp");
    build.file("bgfx/src/glcontext_egl.cpp");
    build.file("bgfx/src/glcontext_glx.cpp");
    build.file("bgfx/src/glcontext_html5.cpp");
    build.file("bgfx/src/glcontext_wgl.cpp");
    build.file("bgfx/src/nvapi.cpp");
    build.file("bgfx/src/renderer_agc.cpp");
    build.file("bgfx/src/renderer_d3d11.cpp");
    build.file("bgfx/src/renderer_d3d12.cpp");
    build.file("bgfx/src/renderer_d3d9.cpp");
    build.file("bgfx/src/renderer_gl.cpp");
    build.file("bgfx/src/renderer_gnm.cpp");
    build.file("bgfx/src/renderer_noop.cpp");
    build.file("bgfx/src/renderer_nvn.cpp");
    build.file("bgfx/src/renderer_vk.cpp");
    build.file("bgfx/src/renderer_webgpu.cpp");
    build.file("bgfx/src/shader.cpp");
    build.file("bgfx/src/shader_dx9bc.cpp");
    build.file("bgfx/src/shader_dxbc.cpp");
    build.file("bgfx/src/shader_spirv.cpp");
    build.file("bgfx/src/topology.cpp");
    build.file("bgfx/src/vertexlayout.cpp");

    build.compile("bgfx_sys");
    // links
    if iswindows {
        println!("cargo:warning=Compiling to Windows");

        println!("cargo:rustc-link-lib=winpthread");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=psapi");        
    } 
    else if isdarwin {
        println!("cargo:warning=Compiling to Darwin");

        println!("cargo:rustc-link-lib=c++");
        
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=AppKit");
        // println!("cargo:rustc-link-lib=framework=Metal");
    } 
    else if isunix {
        println!("cargo:warning=Compiling to Unix (linux)");
        
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=X11");
    }
    else {
        panic!("OS not suported")
    }
    
    println!("cargo:warning=All Done");
}
