mod common;
use common::{get_render_type, get_platform_data};
use bgfx_rs::*;
use glfw::{Action, Key, WindowHint, ClientApiHint};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[repr(packed)]
struct PosColorVertex {
    _x: f32,
    _y: f32,
    _z: f32,
}

static VERTICES: [PosColorVertex; 3] = [
    PosColorVertex { _y: -0.5,  _x: -0.5, _z: 0.0}, // 0
    PosColorVertex { _y:  0.5,  _x: -0.5, _z: 0.0}, // 1
    PosColorVertex { _y:  0.5,  _x:  0.5, _z: 0.0}, // 2
];


static INDICES: [u32; 3] = [
    0, 1, 2,
];
pub fn load_shader_file(name: &str) -> std::io::Result<Vec<u8>> {

    let ext = match bgfx::get_renderer_type() {
        // RendererType::Direct3D11 => path.push("d11"),
        RendererType::OpenGL => "gl",
        RendererType::Metal => "mt",
        // RendererType::OpenGLES => path.push("el"),
        RendererType::Vulkan => "vk",
        e => panic!("Unsupported render type {:#?}", e),
    };

    let mut data = std::fs::read(format!("../resources/triangle/{}.{}", name, ext))?;
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



pub fn main() -> std::io::Result<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi)); // solves memory leek

    let (mut window, events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "triangle.rs bgfx-rs example - ESC to close",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);


    // initialization of the library
    let mut init = Init::new();
    init.type_r = get_render_type();
    init.resolution.reset = ResetFlags::VSYNC.bits();
    init.debug = false;
    init.resolution.reset = ResetFlags::VSYNC.bits();
    init.platform_data = get_platform_data(&window);

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
        layout.end();

        let verts_mem = Memory::reference(&VERTICES);
        let index_mem = Memory::reference(&INDICES);

        let vbh = bgfx::create_vertex_buffer(&verts_mem, &layout, BufferFlags::NONE.bits());
        let ibh =  bgfx::create_index_buffer(&index_mem, BufferFlags::NONE.bits());

        let u_color = Uniform::create("u_color", UniformType::Vec4, 1);
        let shader_program = load_shader_program("vs_triangle", "fs_triangle")?;

        let mut count = 0.0;
        let mut increment = 0.05;
        let mut frame = 0;
        let mut old_size = (0, 0);
        
        bgfx::touch(0);
        while !window.should_close() {
            // Importnat function
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

            // view information
            bgfx::set_view_rect(0, 0, 0, size.0 as u16, size.1 as u16);

            // color of triangles
            let u_color_data = [count, 0.5, 1.0, 1.0];
            frame += 1;
            if frame == 150 {
                if count > 1.0 {
                    increment = -0.05;
                }
                else if count < 0.0 {
                    increment = 0.05;
                }
                count += increment;
                frame = 0;
            }

            bgfx::set_uniform(&u_color, &u_color_data, 1);

            // triangle
            bgfx::set_vertex_buffer(0, &vbh, 0, VERTICES.len() as u32);
            bgfx::set_index_buffer(&ibh, 0, (INDICES.len() * 4) as u32);
            bgfx::submit(0, &shader_program, SubmitArgs::default());

            // rendering
            bgfx::frame(false);
        }
    }

    bgfx::shutdown();

    Ok(())
}