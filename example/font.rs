use std::vec;

use bgfx::*;
use bgfx_rs::bgfx;
use font_kit::{source::SystemSource, canvas::{Canvas, RasterizationOptions, Format}, sources::fs, hinting::HintingOptions, properties::Properties, family_name::FamilyName};
use glam::{Vec3, Mat4, Mat3};
use pathfinder_geometry::{vector::{Vector2I, Vector2F}, transform2d::Transform2F};
use glfw::{Action, Context, Key};

mod common;
use common::{get_render_type, get_platform_data};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;


fn render_text() -> Vec<u8> {
    let font = SystemSource::new().select_best_match(&[FamilyName::Monospace],
        &Properties::new())
        .unwrap()
        .load()
        .unwrap();

    let mut canvas = Canvas::new(Vector2I::splat(32), Format::A8);
    let mut pixels: Vec<u8> = vec![];

    for char in String::from("Hello World").chars() {
        println!("Rendering {}", &char);

        let glyph_id = font.glyph_for_char(char).unwrap();
        font.rasterize_glyph(&mut canvas,
            glyph_id,
            32.0,
            Transform2F::from_translation(Vector2F::new(0.0, 32.0)),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa)
            .unwrap();
        pixels.extend(canvas.pixels.clone())
    }

    pixels
}

pub fn main() -> std::io::Result<()>  {
    let text = render_text();
    let text_ref = Memory::reference(&text);

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

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
    let mut old_size = (0, 0);

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
            old_size = size;
        }

        let aspect = size.0 as f32 / size.1 as f32;

        let proj_mtx = Mat4::perspective_lh(60.0 * (std::f32::consts::PI / 180.0), aspect, 0.1, 100.0);
        let view_mtx = Mat4::look_at_lh(eye, at, up);

        bgfx::set_view_rect(0, 0, 0, size.0 as _, size.1 as _);
        bgfx::touch(0);

        bgfx::set_view_transform(0, &view_mtx.to_cols_array(), &proj_mtx.to_cols_array());

        let x = -2.0;
        let y = 0.0;

        // https://github.com/bkaradzic/bgfx/blob/master/examples/common/font/text_buffer_manager.cpp#L974-L1191
        // bgfx::set_texture(stage, sampler, handle, flags)

        let texture = bgfx::create_texture_2d(32 * 11, 32, false, 0, TextureFormat::A8, TextureFlags::SRGB.bits(), &text_ref);
      
        bgfx::set_texture(0, &Uniform::from(0), &texture, TextureFlags::SRGB.bits());

        bgfx::frame(false);
    }

    bgfx::shutdown();

    Ok(())
}
