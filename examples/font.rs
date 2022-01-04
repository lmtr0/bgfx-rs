use std::vec;

use bgfx::*;
use bgfx_rs::bgfx;
use freetype::{Library, face::LoadFlag, Bitmap};
use glam::{Vec3, Mat4, Mat3};
use glfw::{Action, Context, Key, WindowHint, ClientApiHint};


mod common;
use common::{get_render_type, get_platform_data};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;


fn render_text<S: AsRef<str>>(text: S) -> Bitmap {
    let text = text.as_ref();
    
    // Init the library
    let lib = Library::init().unwrap();
    // Load a font face
    let face = lib.new_face("./fonts/Nunito-Regular.ttf", 0).unwrap();
    // Set the font size
    face.set_char_size(32 * text.len() as isize, 0, 50, 10).unwrap();
    // Load a character
    for char in text.chars() {
        face.load_char(char as usize, LoadFlag::RENDER).unwrap();
    }
    
    
    // Get the glyph instance
    let glyph = face.glyph();
    let bitmap = glyph.bitmap();

    bitmap
}

pub fn load_shader_file(name: &str) -> std::io::Result<Vec<u8>> {

    let ext = match bgfx::get_renderer_type() {
        // RendererType::Direct3D11 => path.push("d11"),
        RendererType::OpenGL => "gl",
        // RendererType::Metal => path.push("mt"),
        // RendererType::OpenGLES => path.push("el"),
        RendererType::Vulkan => "vk",
        e => panic!("Unsupported render type {:#?}", e),
    };

    let mut data = std::fs::read(format!("./font/{}.{}", name, ext))?;
    data.push(0); // this is to terminate the data
    Ok(data)
}

// load shaders and create shader program
pub fn load_shader_program(vs: &str, ps: &str) -> std::io::Result<Program> {
    let vs_data = load_shader_file(vs)?;
    let ps_data = load_shader_file(ps)?;

    let vs_data = Memory::copy(&vs_data);
    let ps_data = Memory::copy(&ps_data);

    let vs_shader = bgfx::create_shader(&vs_data);
    let ps_shader = bgfx::create_shader(&ps_data);

    Ok(bgfx::create_program(&vs_shader, &ps_shader, true))
}


pub fn main() -> std::io::Result<()>  {
    let text = render_text("Hello World");

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "font.rs bgfx-rs example - ESC to close",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    let pd = get_platform_data(&window);
    let mut init = Init::new();

    init.type_r = get_render_type();
    init.resolution.width = WIDTH as u32;
    init.resolution.height = HEIGHT as u32;
    init.resolution.reset = ResetFlags::NONE.bits();
    init.platform_data = pd;

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }

    bgfx::set_debug(DebugFlags::TEXT.bits());
    bgfx::set_view_clear(
        0,
        ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
        SetViewClearArgs {
            rgba: 0x103030ff,
            ..Default::default()
        },
    );
    let at = Vec3::new(0.0, 0.0, 0.0);
    //                                        this controls the width of the view
    let eye = Vec3::new(0.0, 0.0, -5.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let mut old_size = window.get_size();


    let s_texColor = bgfx::Uniform::create("s_texColor", bgfx::UniformType::Vec4, 1);
	// let u_dropShadowColor = bgfx::Uniform::create("u_dropShadowColor", bgfx::UniformType::Vec4, 1);
	// let u_params = bgfx::Uniform::create("u_params", bgfx::UniformType::Vec4, 1);
    let shader = load_shader_program("vs_font", "fs_font").unwrap();
    let state = (StateWriteFlags::R
        | StateWriteFlags::G
        | StateWriteFlags::B
        | StateWriteFlags::A
        | StateWriteFlags::Z)
        .bits()
        | StateDepthTestFlags::LESS.bits()
        | StateCullFlags::CW.bits();

    bgfx::set_view_rect(0, 0, 0, old_size.0 as _, old_size.1 as _);
    bgfx::touch(0);


    while !window.should_close() {
        glfw.poll_events();


        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
        }

        let size = window.get_framebuffer_size();

        if old_size != size {
            bgfx::reset(size.0 as _, size.1 as _, ResetArgs::default());
            bgfx::set_view_rect(0, 0, 0, size.0 as _, size.1 as _);
            bgfx::touch(0);

            old_size = size;
        }

        let aspect = size.0 as f32 / size.1 as f32;

        let proj_mtx = Mat4::perspective_lh(60.0 * (std::f32::consts::PI / 180.0), aspect, 0.1, 100.0);
        let view_mtx = Mat4::look_at_lh(eye, at, up);
        
        let x = -2.0;
        let y = 0.0;
        let transform = Mat4::from_translation(Vec3::new(x, y, 0.0));


        bgfx::set_view_transform(0, &view_mtx.to_cols_array(), &proj_mtx.to_cols_array());
        bgfx::set_transform(&transform.to_cols_array(), 1);
        bgfx::set_uniform(&s_texColor, &[1.0, 1.0, 1.0, 1.0], 1);

        bgfx::set_state(state, 0);
        bgfx::submit(0, &shader, SubmitArgs::default());



        // https://github.com/bkaradzic/bgfx/blob/master/examples/common/font/text_buffer_manager.cpp#L974-L1191
        // bgfx::set_texture(stage, sampler, handle, flags)

        
        bgfx::frame(false);
    }

    bgfx::shutdown();

    Ok(())
}
