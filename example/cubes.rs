use bgfx_rs::*;
use glam::{EulerRot, Mat4, Vec3};
use glfw::{Action, Context, Key, WindowHint, ClientApiHint};
use std::{time::Instant};

mod common;
use common::{get_platform_data, get_render_type, _load_shader_program};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[repr(packed)]
struct PosColorVertex {
    _x: f32,
    _y: f32,
    _z: f32,
    _abgr: u32,
}

#[rustfmt::skip]
static CUBE_VERTICES: [PosColorVertex; 8] = [
    PosColorVertex { _x: -1.0, _y:  1.0, _z:  1.0, _abgr: 0xff000000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z:  1.0, _abgr: 0xff0000ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ffff },
    PosColorVertex { _x: -1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff0000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff00ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffffff },
];

#[rustfmt::skip]
static CUBE_INDICES: [u16; 36] = [
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

pub fn main() -> std::io::Result<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "cubes.rs bgfx-rs example - ESC to close",
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

    {
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

        let verts_mem = Memory::reference(&CUBE_VERTICES);
        let index_mem = Memory::reference(&CUBE_INDICES);

        let vbh = bgfx::create_vertex_buffer(&verts_mem, &layout, BufferFlags::NONE.bits());
        let ibh = bgfx::create_index_buffer(&index_mem, BufferFlags::NONE.bits());

        let shader_program = _load_shader_program("vs_cubes", "fs_cubes")?;

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

        let mut old_size = (0, 0);

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    window.set_should_close(true)
                }
            }

            let t = time.elapsed().as_secs_f32();
            let size = window.get_framebuffer_size();

            if old_size != size {
                bgfx::reset(size.0 as _, size.1 as _, ResetArgs::default());
                old_size = size;
            }

            let aspect = size.0 as f32 / size.1 as f32;

            let persp =
                Mat4::perspective_lh(60.0 * (std::f32::consts::PI / 180.0), aspect, 0.1, 100.0);
            let view = Mat4::look_at_lh(eye, at, up);

            bgfx::set_view_rect(0, 0, 0, size.0 as _, size.1 as _);
            bgfx::touch(0);

            bgfx::set_view_transform(0, &view.to_cols_array(), &persp.to_cols_array());

            for yy in 0..11 {
                for xx in 0..11 {
                    let x = -15.0 + (xx as f32) * 3.0;
                    let y = -15.0 + (yy as f32) * 3.0;
                    let xr = t + (xx as f32) * 0.21;
                    let yr = t + (yy as f32) * 0.37;

                    let rot = Mat4::from_euler(EulerRot::XYZ, xr, yr, 0.0);
                    let transform = Mat4::from_translation(Vec3::new(x*2.0, y, 0.0)) * rot;

                    bgfx::set_transform(&transform.to_cols_array(), 1);
                    bgfx::set_vertex_buffer(0, &vbh, 0, std::u32::MAX);
                    bgfx::set_index_buffer(&ibh, 0, std::u32::MAX);

                    bgfx::set_state(state, 0);
                    bgfx::submit(0, &shader_program, SubmitArgs::default());
                }
            }

            bgfx::frame(false);
        }
    }

    bgfx::shutdown();

    Ok(())
}
