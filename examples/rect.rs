
use bgfx_rs::*;
use glfw::{Action, Key, WindowHint, ClientApiHint};

mod common;
use common::{get_render_type, get_platform_data};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[repr(packed)]
struct PosVertex {
    _x: f32,
    _y: f32,
}

static VERTICES: [PosVertex; 4] = [
    PosVertex { _y: -0.5,  _x: -0.5}, // 0
    PosVertex { _y:  0.5,  _x: -0.5}, // 1
    PosVertex { _y:  0.5,  _x:  0.5}, // 2
    PosVertex { _y: -0.5,  _x:  0.5}, // 3
];


static INDICES: [u32; 6] = [
    0, 1, 2,
    2, 3, 0
];

pub fn load_shader_file(name: &str) -> std::io::Result<Vec<u8>> {

    let ext = match bgfx::get_renderer_type() {
        // RendererType::Direct3D11 => path.push("d11"),
        RendererType::OpenGL => "gl",
        // RendererType::Metal => path.push("mt"),
        // RendererType::OpenGLES => path.push("el"),
        RendererType::Vulkan => "vk",
        e => panic!("Unsupported render type {:#?}", e),
    };

    let mut data = std::fs::read(format!("./resources/rect/{}.{}", name, ext))?;
    data.push(0); // this is to terminate the data
    Ok(data)
}

// load shaders and create shader program
pub fn load_shader_program(vs: &str, fs: &str) -> std::io::Result<Program> {
    let vs_data = load_shader_file(vs)?;
    let fs_data = load_shader_file(fs)?;

    let vs_data = Memory::copy(&vs_data);
    let fs_data = Memory::copy(&fs_data);

    let vs_shader = bgfx::create_shader(&vs_data);
    let fs_shader = bgfx::create_shader(&fs_data);

    Ok(bgfx::create_program(&vs_shader, &fs_shader, true))
}

pub fn main() -> std::io::Result<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "rect.rs bgfx-rs example - ESC to close",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);


    let mut init = Init::new();

    init.type_r = get_render_type();
    init.resolution.reset = ResetFlags::NONE.bits();
    init.resolution.reset = ResetFlags::NONE.bits();
    init.platform_data.nwh = get_platform_data(&window).nwh;
    init.platform_data.ndt = get_platform_data(&window).ndt;

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
        layout.add(Attrib::Position, 2, AttribType::Float, AddArgs::default());
        layout.end();

        let verts_mem = Memory::reference(&VERTICES);
        let index_mem = Memory::reference(&INDICES);

        let vbh = bgfx::create_vertex_buffer(&verts_mem, &layout, BufferFlags::NONE.bits());
        let ibh =  bgfx::create_index_buffer(&index_mem, BufferFlags::NONE.bits());

        let u_color = Uniform::create("u_color", UniformType::Vec4, 1);
        let shader_program = load_shader_program("vs_rect", "fs_rect")?;
        let state = (StateWriteFlags::R
            | StateWriteFlags::G
            | StateWriteFlags::B
            | StateWriteFlags::A
            | StateWriteFlags::Z)
            .bits()
            | StateDepthTestFlags::LESS.bits()
            | StateCullFlags::CW.bits();

        let mut old_size = (0, 0);
        
        while !window.should_close() {
            // glfw.wait_events();
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    window.set_should_close(true)
                }
            }
            
            bgfx::touch(0);
            let size = window.get_framebuffer_size();

            if old_size != size {
                bgfx::reset(size.0 as _, size.1 as _, ResetArgs::default());
                old_size = size;
            }

            
            let data = [0.5, 0.5, 1.0, 1.0];
            // frame += 1;
            // if frame == 150 {
            //     if count > 1.0 {
            //         increment = -0.05;
            //     }
            //     else if count < 0.0 {
            //         increment = 0.05;
            //     }
            //     count += increment;
            //     frame = 0;
            // }

            // viewport
            bgfx::set_view_rect(0, 0, 0, size.0 as u16, size.1 as u16);
            
            // geometry and position
            bgfx::set_vertex_buffer(0, &vbh, 0, std::u32::MAX);
            bgfx::set_index_buffer(&ibh, 0, std::u32::MAX);
            
            // color
            bgfx::set_uniform(&u_color, &data, 1);
            
            // renders
            bgfx::set_state(state, 0);
            bgfx::submit(0, &shader_program, SubmitArgs::default());
            bgfx::frame(false);
        }
    }

    bgfx::shutdown();

    Ok(())
}
