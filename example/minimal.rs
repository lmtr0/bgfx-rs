use std::ffi::c_void;

use glfw::{self, Key, Action, ClientApiHint, WindowHint};
use bgfx_rs::*;
mod common;
use common::{get_platform_data, get_render_type};
fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Error initializing library");
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

    let (mut window, events) = glfw.create_window(
        1080 as _, 
        900 as _, 
        "Window 1", 
        glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window.");
    
    window.set_pos(200, 200);
    window.set_size_polling(true);

    window.set_key_polling(true);

    let mut init = Init::new();
    init.type_r = get_render_type();
    // init.resolution.height = 0;
    // init.resolution.width = 0;
    init.resolution.reset = ResetFlags::NONE.bits(); // this makes the window recreation smoth
    init.platform_data = get_platform_data(&window);

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }


    let mut fb_size = window.get_size();
    let mut fb = bgfx::create_frame_buffer_from_nwh(
        get_platform_data(&window).nwh as *mut c_void,
        fb_size.0 as u16,
        fb_size.1 as u16,
        CreateFrameBufferFromNwhArgs::default(),
    );

    let mut should_close = false;
    while !should_close {
        glfw.wait_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                should_close = true;
            }
        }
        let idx = 0;

        let size = window.get_framebuffer_size();
        
        if fb_size != size {
            fb = bgfx::create_frame_buffer_from_nwh(
                get_platform_data(&window).nwh as *mut c_void,
                size.0 as u16,
                size.1 as u16,
                CreateFrameBufferFromNwhArgs::default(),
            );
            fb_size = size;
        }
        
        bgfx::set_view_frame_buffer(idx as _, &fb);
        
        bgfx::touch(idx as _);
        let color = 0x103030ff;
        
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

        bgfx::frame(false);
    } // end main loop

    drop(fb);
    bgfx::shutdown();
}