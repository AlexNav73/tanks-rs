
#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::{Device, Encoder};
use gfx::traits::FactoryExt;
use glutin::{WindowEvent, VirtualKeyCode};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] }
];

pub fn main() {

    let event_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Tiangle example".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &event_loop);

    let mut encoder: Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        include_bytes!("../assets/shaders/triangle_150.glslv"),
        include_bytes!("../assets/shaders/triangle_150.glslf"),
        pipe::new()
    ).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color
    };

    let mut running = true;
    while running {
        event_loop.poll_events(|glutin::Event::WindowEvent { window_id: _, event }| {
            match event {
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Escape), _) |
                WindowEvent::Closed => running = false,
                WindowEvent::Resized(_w, _h) => gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth),
                _ => {}
            }
        });

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }

}
