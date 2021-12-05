use bgfx_rs::{FrameBuffer, create_vertex_buffer};
use bgfx_rs::bgfx;
use bgfx::*;
use core::ffi::c_void;
use std::path::{PathBuf, Path};
use glfw::{Action, Key, Window};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

#[repr(packed)]
struct PosColorVertex {
    _x: f32,
    _y: f32,
    _abgr: u32,
}

static VERTICES: [PosColorVertex; 3] = [
    PosColorVertex { _x: -0.5, _y: -0.5, _abgr: 0xff000000 },
    PosColorVertex { _x:  0.0, _y:  0.5, _abgr: 0xff0000ff },
    PosColorVertex { _x:  0.5, _y: -0.5, _abgr: 0xff00ff00 },
];


fn render_triangle() {
    let ver_ref = unsafe {Memory::reference(&VERTICES)};
    let layout = VertexLayoutBuilder::new();
    layout.begin(RendererType::Vulkan);
    layout.add(Attrib::Position, 2, AttribType::Float, AddArgs::default());
    layout.add(
        Attrib::Color0,
        4,
        AttribType::Uint8,
        AddArgs {
            normalized: true,
            as_int: false,
        },
    );
    layout.end();

    let vbh = bgfx::create_vertex_buffer(&ver_ref, &layout, BufferFlags::NONE.bits());
    let state = (StateWriteFlags::R
        | StateWriteFlags::G
        | StateWriteFlags::B
        | StateWriteFlags::A
        | StateWriteFlags::Z)
        .bits()
        | StateDepthTestFlags::LESS.bits()
        | StateCullFlags::CW.bits();

    bgfx::set_vertex_buffer(0, &vbh, 0, std::u32::MAX);
    bgfx::set_state(state, 0);
}


fn load_shader_file(name: &str) -> std::io::Result<Vec<u8>> {
    let mut path = Path::new("resources/");

    // match bgfx::get_renderer_type() {
    //     RendererType::Direct3D11 => path.push("dx11"),
    //     RendererType::OpenGL => path.push("glsl"),
    //     RendererType::Metal => path.push("metal"),
    //     RendererType::OpenGLES => path.push("essl"),
    //     RendererType::Vulkan => path.push("spirv"),
    //     e => panic!("Unsupported render type {:#?}", e),
    // }

    let path = path.join(format!("{}.bin", name));

    let mut data = std::fs::read(path)?;
    data.push(0); // this is to terminate the data
    Ok(data)
}


// load shaders and create shader program
fn load_shader_program(vs: &str, ps: &str) -> std::io::Result<Program> {
    let vs_data = load_shader_file(vs)?;
    let ps_data = load_shader_file(ps)?;

    let vs_data = Memory::copy(&vs_data);
    let ps_data = Memory::copy(&ps_data);

    let vs_shader = bgfx::create_shader(&vs_data);
    let ps_shader = bgfx::create_shader(&ps_data);

    Ok(bgfx::create_program(&vs_shader, &ps_shader, false))
}

fn get_platform_data(window: &Window) -> PlatformData {
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

fn get_render_type() -> RendererType {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    return RendererType::OpenGL;
    #[cfg(target_os = "macos")]
    return RendererType::Metal;
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Error initializing library");

    let (mut window, events) = glfw
        .create_window(1080 as _, 900 as _, "Window 1", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_pos(200, 200);
    window.set_size_polling(true);

    window.focus();

    window.set_key_polling(true);

    let mut init = Init::new();
    init.type_r = get_render_type();
    init.resolution.reset = ResetFlags::NONE.bits(); // this makes the window recreation smoth
    init.platform_data = get_platform_data(&window);

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }

    let mut should_close = false;

    let mut size = window.get_size();
    let mut framebuff = create_frame_buffer_from_nwh(
        get_platform_data(&window).nwh as *mut c_void,
        size.0 as u16,
        size.1 as u16,
        CreateFrameBufferFromNwhArgs::default()
    );

    while !should_close {
        glfw.wait_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                should_close = true;
            }

            if let glfw::WindowEvent::Size(width, height) = event {
                framebuff = create_frame_buffer_from_nwh(
                    get_platform_data(&window).nwh as *mut c_void,
                    width as u16,
                    height as u16,
                    CreateFrameBufferFromNwhArgs::default()
                );

                size.0 = width;
                size.1 = height;

                println!("you resized your window");
            }
        }

        bgfx::set_view_frame_buffer(0, &framebuff);
        
        bgfx::touch(0 as _);
        let color = if 0 & 1 == 0 { 0x103030ff } else { 0x755413ff };
        
        bgfx::set_view_rect(0 as _, 0, 0, size.0 as _, size.1 as _);
        bgfx::set_view_clear(
            0 as _,
            ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
            SetViewClearArgs {
                rgba: color,
                depth: 1.0,
                stencil: 0,
            },
        );

        render_triangle();
    
        bgfx::frame(true);
    } // end main loop

    drop(framebuff);

    bgfx::shutdown();
}