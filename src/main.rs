
#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate genmesh;
extern crate cgmath;

use gfx::{Device, Encoder, texture};
use gfx::traits::FactoryExt;
use glutin::{WindowEvent, VirtualKeyCode};
use gfx_core::Factory;

use genmesh::generators::{Cube, IndexedPolygon, SharedVertex};
use genmesh::{Triangulate, Vertices};

use cgmath::{Matrix4, Deg, Point3, Vector3};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),

        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",


        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    fn new(p: [f32; 3], t: [i8; 2]) -> Vertex {
        Vertex {
            pos: [p[0], p[1], p[2], 1.0],
            tex_coord: [t[0] as f32, t[1] as f32],
        }
    }
}

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn main() {
    let event_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Tiangle example".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &event_loop);

    let mut encoder: Encoder<_, _> = factory.create_command_buffer().into();

    let cube = Cube::new();
    let vertex_data: Vec<Vertex> = cube.shared_vertex_iter()
        .map(|v| Vertex::new([v.pos[0], v.pos[1], v.pos[2]], [0, 0]))
        .collect();
    let index_data: Vec<u32> = cube.indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|i| i as u32)
        .collect();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data.as_slice());
    let texels = [[0x20, 0xA0, 0xC0, 0x00]];

    let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
        texture::Kind::D2(1, 1, texture::AaMode::Single), &[&texels]
    ).unwrap();

    let sinfo = texture::SamplerInfo::new(
        texture::FilterMethod::Bilinear,
        texture::WrapMode::Clamp);

    let pso = factory.create_pipeline_simple(
        include_bytes!("../assets/shaders/plane_150.glslv"),
        include_bytes!("../assets/shaders/plane_150.glslf"),
        pipe::new()
    ).unwrap();

    let proj = cgmath::perspective(Deg(45.0f32), 1.333, 1.0, 10.0);

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        transform: (proj * default_view()).into(),
        locals: factory.create_constant_buffer(1),
        color: (texture_view, factory.create_sampler(sinfo)),
        out_color: main_color,
        out_depth: main_depth
    };

    let mut running = true;
    let mut x = 0.0;
    let mut y = 3.0;
    while running {
        event_loop.poll_events(|glutin::Event::WindowEvent { window_id: _, event }| {
            match event {
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Escape), _) |
                WindowEvent::Closed => running = false,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::A), _) => {
                    x += 0.01;
                    x = x % 5.0;
                },
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::D), _) => {
                    x -= 0.01;
                    x = x % -5.0;
                },
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::W), _) => {
                    y += 0.01;
                    y = y % 5.0;
                },
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::S), _) => {
                    y -= 0.01;
                    y = y % -5.0;
                },
                WindowEvent::Resized(_w, _h) => gfx_window_glutin::update_views(&window, &mut data.out_color, &mut data.out_depth),
                _ => {}
            }
        });

        let matrix = Matrix4::look_at(
            Point3::new(x, y, 3.0),
            Point3::new(0.0f32, 0.0, 0.0),
            Vector3::unit_z()
        );
        
        let locals = Locals {
            transform: (proj * matrix).into()
        };

        encoder.clear(&data.out_color, CLEAR_COLOR);
        encoder.clear_depth(&data.out_depth, 1.0);
        encoder.update_constant_buffer(&data.locals, &locals);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

fn default_view() -> Matrix4<f32> {
    Matrix4::look_at(
        Point3::new(1.5f32, -5.0, 3.0),
        Point3::new(0.0f32, 0.0, 0.0),
        Vector3::unit_z()
    )
}
