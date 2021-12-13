use glfw;
use bgfx_rs::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(
            200 as _,
            100 as _,
            "Minimal Example",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_size_polling(true);

    // BGFX

    bgfx::init(&Init::new());


    loop {
        glfw.wait_events();
        let events = glfw::flush_messages(&events);
        
        for (_winid, event) in events {
            if let glfw::WindowEvent::Size(w, h)  = event {
                println!("Resized window to {}x{}", w, h);
            }
        }
    }
}