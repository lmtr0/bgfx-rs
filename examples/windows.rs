use bgfx::*;
use bgfx_rs::bgfx;
use glam::{EulerRot, Mat4, Vec3};
use core::ffi::c_void;
use std::{convert::TryInto, path::PathBuf, time::Instant};
use glfw::{Action, Key, Window, WindowEvent};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

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
    return RendererType::Vulkan;
    #[cfg(target_os = "macos")]
    return RenderType::Metal;
}

#[repr(packed)]
struct PosColorVertex {
    _x: f32,
    _y: f32,
    _z: f32,
    _abgr: u32,
}

fn load_shader_file(name: &str) -> std::io::Result<Vec<u8>> {
    let mut path = PathBuf::with_capacity(512);
    path.push("resources/examples/runtime/shaders");

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

fn load_shader_program(vs: &str, ps: &str) -> std::io::Result<Program> {
    let vs_data = load_shader_file(vs)?;
    let ps_data = load_shader_file(ps)?;

    let vs_data = Memory::copy(&vs_data);
    let ps_data = Memory::copy(&ps_data);

    let vs_shader = bgfx::create_shader(&vs_data);
    let ps_shader = bgfx::create_shader(&ps_data);

    Ok(bgfx::create_program(&vs_shader, &ps_shader, false))
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Error initializing library");

    let (mut window, events) = glfw
        .create_window(1080 as _, 900 as _, "Window 1", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    let (mut window2, events2) = glfw
        .create_window(1080 as _, 900 as _, "Window 2", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_pos(100, 100);
    window.set_size_polling(true);

    window2.set_pos(1080 + 200, 100);
    window2.set_size_polling(true);

    window.focus();

    window.set_key_polling(true);
    window2.set_key_polling(true);

    let mut init = Init::new();
    init.type_r = get_render_type();
    init.resolution.height = 0;
    init.resolution.width = 0;
    init.resolution.reset = ResetFlags::VSYNC.bits(); // this makes the window recreation smoth
    init.platform_data = get_platform_data(&window);

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }


    let caps = bgfx::get_caps();
    println!("rendering with {:?}", caps.renderer_type);

    let mut framebuffer = bgfx::create_frame_buffer_from_nwh(
        get_platform_data(&window).nwh as *mut c_void,
        window.get_size().0 as u16,
        window.get_size().1 as u16,
        CreateFrameBufferFromNwhArgs::default(),
    );

    let mut framebuffer2 = bgfx::create_frame_buffer_from_nwh(
        get_platform_data(&window2).nwh as *mut c_void,
        window2.get_size().0 as u16,
        window2.get_size().1 as u16,
        CreateFrameBufferFromNwhArgs::default(),
    );

    let windows = [window, window2];

    let mut should_close = false;
    let verticies_cubes: [PosColorVertex; 8] = [
        PosColorVertex { _x: -1.0, _y:  1.0, _z:  1.0, _abgr: 0xff000000 },
        PosColorVertex { _x:  1.0, _y:  1.0, _z:  1.0, _abgr: 0xff0000ff },
        PosColorVertex { _x: -1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ff00 },
        PosColorVertex { _x:  1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ffff },
        PosColorVertex { _x: -1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff0000 },
        PosColorVertex { _x:  1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff00ff },
        PosColorVertex { _x: -1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffff00 },
        PosColorVertex { _x:  1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffffff },
    ];
    
    
    let indecies_cube: [u16; 36] = [
        0, 1, 2, // 0
        1, 3, 2,
        4, 6, 5, // 2
        5, 6, 7,
        0, 2, 4, // 4
        4, 2, 6,
        1, 5, 3, // 6
        5, 7, 3,
        0, 4, 1, // 8
        4, 5, 1,
        2, 3, 6, // 10
        6, 3, 7,
    ];

    let layout = VertexLayoutBuilder::new();
    layout.begin(RendererType::Noop);
    layout.add(Attrib::Position, 3, AttribType::Float, AddArgs::default());
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

    let verts_mem = unsafe { Memory::reference(&verticies_cubes) };
    let index_mem = unsafe { Memory::reference(&indecies_cube) };

    let vbh = bgfx::create_vertex_buffer(&verts_mem, &layout, BufferFlags::NONE.bits());
    let ibh = bgfx::create_index_buffer(&index_mem, BufferFlags::NONE.bits());

    let shader_program = load_shader_program("vs_cubes", "fs_cubes").unwrap();

    let state = (StateWriteFlags::R
        | StateWriteFlags::G
        | StateWriteFlags::B
        | StateWriteFlags::A
        | StateWriteFlags::Z)
        .bits()
        | StateDepthTestFlags::LESS.bits()
        | StateCullFlags::CW.bits();

    let at = Vec3::new(0.0, 0.0, 0.0);
    let eye = Vec3::new(0.0, 0.0, -35.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let time = Instant::now();

    while !should_close {
        glfw.poll_events();
        // first window
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                should_close = true;
            }

            if let WindowEvent::Size(_, _) = event {
                let window = &windows[0];

                framebuffer = bgfx::create_frame_buffer_from_nwh(
                    get_platform_data(&window).nwh as *mut c_void,
                    window.get_size().0 as u16,
                    window.get_size().1 as u16,
                    CreateFrameBufferFromNwhArgs::default(),
                );
            }
        }

        // second window
        for (_, event) in glfw::flush_messages(&events2) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                should_close = true;
            }

            if let WindowEvent::Size(width, height) = event {
                let window = &windows[1];

                framebuffer2 = bgfx::create_frame_buffer_from_nwh(
                    get_platform_data(&window).nwh as *mut c_void,
                    width as u16,
                    height as u16,
                    CreateFrameBufferFromNwhArgs::default(),
                );
            }
        }

        let mut idx = 0;

        for window in windows.iter() {
            let id: u16 = idx;
            let color = if idx & 1 == 0 { 0x103030ff } else { 0x755413ff };

            if id == 0 {
                bgfx::set_view_frame_buffer(id, &framebuffer);
            } else {
                bgfx::set_view_frame_buffer(id, &framebuffer2);
            }
            let size = window.get_framebuffer_size();

            // bgfx::reset(size.0 as _, size.1 as _, ResetArgs::default());
            bgfx::touch(id);
            bgfx::set_view_rect(id, 0, 0, size.0 as _, size.1 as _);
            bgfx::set_view_clear(id, ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
            SetViewClearArgs {
                rgba: color,
                depth: 1.0,
                stencil: 0,
            });
            let aspect = size.0 as f32 / size.1 as f32;
            let t = time.elapsed().as_secs_f32();
            let persp =
                Mat4::perspective_lh( 30.0 * (std::f32::consts::PI / 180.0), aspect, 1.0, 100.0);
            let view = Mat4::look_at_lh(eye, at, up);

            bgfx::set_view_transform(id.try_into().unwrap(), &view.to_cols_array(), &persp.to_cols_array());

            for yy in 0..2 {
                for xx in -1..2 {
                    let x = xx as f32 * 3.0;
                    let y = yy as f32 * 3.0;
                    let xr = t + (xx as f32);
                    let yr = t + (yy as f32);

                    let rot = Mat4::from_euler(EulerRot::XYZ, xr, yr, 0.0);
                    let transform = Mat4::from_translation(Vec3::new(x, y, 0.0)) * rot;

                    bgfx::set_transform(&transform.to_cols_array(), 1);
                    bgfx::set_vertex_buffer(id.try_into().unwrap(), &vbh, 0, std::u32::MAX);
                    bgfx::set_index_buffer(&ibh, 0, std::u32::MAX);

                    bgfx::set_state(state, 0);
                    bgfx::submit(id, &shader_program, SubmitArgs::default());
                }
            }
            idx += 1;
        }

        bgfx::frame(false);
    }

    drop(framebuffer);
    drop(framebuffer2);
    bgfx::shutdown();

    std::process::exit(0); // stops core dump error
}