use bgfx_rs::{FrameBuffer, create_vertex_buffer};
use bgfx_rs::bgfx;
use bgfx::*;
use core::ffi::c_void;
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


pub fn render(framebuffer: FrameBuffer) {
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
    
        bgfx::frame(false);
    } // end main loop

    bgfx::shutdown();
}