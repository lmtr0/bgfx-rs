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

    // ? generate ffi
    // ! broke..
    // let bindings = bindgen::builder()
    //     .layout_tests(false)
    //     .clang_arg("-Ibx/include")

    //     .prepend_enum_name(true)
    //     .allowlist_function("bgfx.*")
    //     .allowlist_type("bgfx.*")
    //     .allowlist_type("BGFX_.*")
    //     .allowlist_var("bgfx.*")
    //     .allowlist_var("BGFX.*")

    //     // specific types that break bindgen
    //     .allowlist_var("BGFX_BUFFER_COMPUTE.*")
    //     .allowlist_var("BGFX_STATE_WRITE_R.*")
    //     .allowlist_var("BGFX_STENCIL_OP_PASS_Z_ZERO.*")
    //     .allowlist_var("BGFX_CLEAR_NONE.*")
    //     .allowlist_var("BGFX_DISCARD_NONE.*")
    //     .allowlist_var("BGFX_BUFFER_NONE.*")
    //     .allowlist_var("BGFX_TEXTURE_NONE.*")
    //     .allowlist_var("BGFX_SAMPLER_NONE.*")
    //     .allowlist_var("BGFX_RESET_NONE.*")
    //     .allowlist_var("BGFX_CAPS_ALPHA_TO_COVERAGE.*")
    //     .allowlist_var("BGFX_RESOLVE_NONE.*")
    //     .allowlist_var("BGFX_PCI_ID_NONE.*")
    //     .allowlist_var("BGFX_CUBE_MAP_POSITIVE_X.*")
    //     .allowlist_var("BGFX_STATE_BLEND_FUNC_RT_1E.*")
        
    //     .header("src/header.h")
    //     .allowlist_recursively(true)
    //     .generate().expect("Failed to generate bindings");

    // bindings
    //     .write_to_file("src/lib.rs")
    //     .expect("Couldn't write bindings!");


    // ! build
    build.define("BGFX_CONFIG_RENDERER_WEBGPU", "0");
    build.define("BGFX_CONFIG_RENDERER_GNM", "0");
    build.define("BIMG_DECODE_ASTC", "0");
    build.define("BGFX_CONFIG_MULTITHREADED", "1"); // maybe

    // disable debug
    build.define("BX_CONFIG_DEBUG", "0");
    build.define("NDEBUG", "1");
    build.debug(false);
    build.opt_level(3);

    if iswindows {
        build.include("bx/include/compat/mingw");
        // build.include("bgfx/3rdparty/dxsdk/include");

        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_DIRECT3D11", "1");
        // build.define("_WIN32", None);
        // build.define("_HAS_EXCEPTIONS", "0");
        // build.define("_SCL_SECURE", "0");
        // build.define("_SECURE_SCL", "0");
        // build.define("__STDC_LIMIT_MACROS", None);
        // build.define("__STDC_FORMAT_MACROS", None);
        // build.define("__STDC_CONSTANT_MACROS", None);
        // build.define("_CRT_SECURE_NO_WARNINGS", None);
        // build.define("_CRT_SECURE_NO_DEPRECATE", None);
        build.warnings(false);
    } 
    else if isdarwin {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "0");
        build.define("BGFX_CONFIG_RENDERER_OPENGL", "0");
        build.define("BGFX_CONFIG_RENDERER_METAL", "1");

        build.include("bx/include/compat/osx");
        
        build.warnings(false);
    }
    else if isunix {
        build.define("BGFX_CONFIG_RENDERER_VULKAN", "1");
        build.define("BGFX_CONFIG_RENDERER_OPENGL", "1");

        build.flag("-mpreferred-stack-boundary=4");
        build.flag("-ffast-math");
        build.flag("-fomit-frame-pointer");
        build.flag("-g");
        build.flag("-O3");
        build.flag("-mfpmath=sse");
        build.flag("-msse2");
        build.flag("-m64");
        build.flag("-fPIE");
        build.flag("-fPIC");
        build.flag("-std=c++14");
        build.flag("-fno-rtti");
        build.flag("-fno-exceptions");
        build.flag("-fno-omit-frame-pointer");
        
        // break the build
        // build.flag("-fsanitize=address");
        // build.flag("-fsanitize=undefined");
        // build.flag("-fsanitize=float-divide-by-zero");
        // build.flag("-fsanitize=float-cast-overflow");
        
        build.warnings(true);
    }


    
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
    build.include("bimg/src");
    build.include("bimg/3rdparty");
    build.include("bimg/3rdparty/iqa/include/");
    build.include("bimg/3rdparty/astc-codec/include");
    build.include("bimg/3rdparty/tinyexr/deps/miniz/");


    // add_all_files("3rdparty/libsquish/*.cpp", &mut build);
    // add_all_files("src/image_encode.*", &mut build);
    // add_all_files("src/image_cubemap_filter.*", &mut build);
    // add_all_files("3rdparty/libsquish/*.cpp", &mut build);
    // add_all_files("3rdparty/libsquish/*.h", &mut build);
    // add_all_files("3rdparty/edtaa3/*.cpp", &mut build);
    // add_all_files("3rdparty/edtaa3/*.h", &mut build);
    // add_all_files("3rdparty/etc1/*.cpp", &mut build);
    // add_all_files("3rdparty/etc1/*.h", &mut build);
    // add_all_files("3rdparty/etc2/*.cpp", &mut build);
    // add_all_files("3rdparty/etc2/*.hpp", &mut build);
    // add_all_files("3rdparty/nvtt/*.cpp", &mut build);
    // add_all_files("3rdparty/nvtt/*.h", &mut build);
    // add_all_files("3rdparty/pvrtc/*.cpp", &mut build);
    // add_all_files("3rdparty/pvrtc/*.h", &mut build);
    // add_all_files("3rdparty/astc/*.cpp", &mut build);
    // add_all_files("3rdparty/astc/*.h", &mut build);
    // add_all_files("3rdparty/tinyexr/*.h", &mut build);
    // add_all_files("3rdparty/iqa/include/*.h", &mut build);
    // add_all_files("3rdparty/iqa/source/*.c", &mut build);
    
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
        
        // println!("cargo:rustc-link-lib=framework=OpenGL"); // deprecated by apple
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");        
    } 
    else if isunix {
        println!("cargo:warning=Compiling to Unix (linux)");
        
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=pthread");
    }
    else {
        panic!("OS not suported")
    }
    
    println!("cargo:warning=All Done");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
