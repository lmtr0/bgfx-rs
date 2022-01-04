use bgfx_rs::{RendererType, PlatformData, Memory, bgfx, Program};
use glfw::Window;
use core::ffi::c_void;
use std::path::PathBuf;
use raw_window_handle::{RawWindowHandle, HasRawWindowHandle};

pub fn get_platform_data(window: &Window) -> PlatformData {
    let mut pd = PlatformData::new();

    match window.raw_window_handle() {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(data) => {
            pd.nwh = data.window as *mut c_void;
            pd.ndt = data.display as *mut c_void;
        }
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(data) => {
            pd.ndt = data.surface; // same as window, on wayland there ins't a concept of windows
            pd.nwh = data.display;
        }

        #[cfg(target_os = "macos")]
        RawWindowHandle::MacOS(data) => {
            pd.nwh = data.ns_window;
        }
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(data) => {
            pd.nwh = data.hwnd;
        }
        _ => panic!("Unsupported Window Manager"),
    }

    return pd;
}

pub fn get_render_type() -> RendererType {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    return RendererType::Vulkan;
    #[cfg(target_os = "macos")]
    return RendererType::Metal;
}

pub fn _load_shader_file(name: &str) -> std::io::Result<Vec<u8>> {
    let mut path = PathBuf::with_capacity(512);
    path.push("./resources/examples/runtime/shaders");

    match bgfx::get_renderer_type() {
        RendererType::Direct3D11 => path.push("dx11"),
        RendererType::OpenGL => path.push("glsl"),
        RendererType::Metal => path.push("metal"),
        RendererType::OpenGLES => path.push("essl"),
        RendererType::Vulkan => path.push("spirv"),
        e => panic!("Unsupported render type {:#?}", e),
    }

    path.push(format!("{}.bin", name));

    let mut data = std::fs::read(path)?;
    data.push(0); // this is to terminate the data
    Ok(data)
}

// load shaders and create shader program
pub fn _load_shader_program(vs: &str, ps: &str) -> std::io::Result<Program> {
    let vs_data = _load_shader_file(vs)?;
    let ps_data = _load_shader_file(ps)?;

    let vs_data = Memory::copy(&vs_data);
    let ps_data = Memory::copy(&ps_data);

    let vs_shader = bgfx::create_shader(&vs_data);
    let ps_shader = bgfx::create_shader(&ps_data);

    Ok(bgfx::create_program(&vs_shader, &ps_shader, false))
}