use bgfx::*;
use bgfx_rs::bgfx;
use core::ffi::c_void;
use glfw::{Action, Key, WindowHint, ClientApiHint};
mod common;
use common::{get_platform_data, get_render_type};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Error initializing library");
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(1080 as _, 900 as _, "Window 1", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    let (mut window2, _events2) = glfw
        .create_window(1080 as _, 900 as _, "Window 2", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_pos(200, 200);
    window.set_size_polling(true);

    window2.set_pos(1080 + 300, 200);
    window2.set_size_polling(true);

    window.focus();

    window.set_key_polling(true);
    window2.set_key_polling(true);

    let mut init = Init::new();
    init.type_r = get_render_type();
    // init.resolution.height = 0;
    // init.resolution.width = 0;
    init.resolution.reset = ResetFlags::NONE.bits(); // this makes the window recreation smoth
    init.platform_data = get_platform_data(&window);

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }

    let windows = [window, window2];
    let mut framebuffers = vec![FrameBuffer::create_frame_buffer_from_nwh(
        get_platform_data(&windows[0]).nwh as *mut c_void,
        windows[0].get_size().0 as u16,
        windows[0].get_size().1 as u16,
        CreateFrameBufferFromNwhArgs::default(),
    ), FrameBuffer::create_frame_buffer_from_nwh(
        get_platform_data(&windows[1]).nwh as *mut c_void,
        windows[1].get_size().0 as u16,
        windows[1].get_size().1 as u16,
        CreateFrameBufferFromNwhArgs::default(),
    )];
    let mut frame_sizes = [(0, 0), (0, 0)];

    let mut should_close = false;
    while !should_close {
        glfw.wait_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                should_close = true;
            }
        }

        for idx in 0..2 {
            
            let window = &windows[idx];
            let size = window.get_framebuffer_size();
            
            if frame_sizes[idx] != size {
                framebuffers[idx] = bgfx::create_frame_buffer_from_nwh(
                    get_platform_data(window).nwh as *mut c_void,
                    window.get_size().0 as u16,
                    window.get_size().1 as u16,
                    CreateFrameBufferFromNwhArgs::default(),
                );

                frame_sizes[idx] = size;
            }
            
            let fb = &framebuffers[idx];
            
            // set the working framebuffer
            bgfx::set_view_frame_buffer(idx.try_into().unwrap(), &fb);
            
            bgfx::touch(idx as _);
            let color = if idx & 1 == 0 { 0x103030ff } else { 0x755413ff };
            
            bgfx::set_view_rect(idx as _, 0, 0, size.0 as _, size.1 as _);
            bgfx::set_view_clear(
                idx as _,
                ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
                SetViewClearArgs {
                    rgba: color,
                    depth: 1.0,
                    stencil: 0,
                },
            );
        }
        
        bgfx::frame(false);
    } // end main loop

    for frame in framebuffers {
        drop(frame)
    }

    bgfx::shutdown();
}