
use gfx;
use glutin;
use cgmath;
use gfx_core;
use gfx_device_gl;
use gfx_window_glutin;

use gfx::{Device, Encoder, texture};
use gfx::traits::FactoryExt;
use gfx_core::Factory;
use glutin::WindowEvent;

use cgmath::{Matrix4, Deg, Point3, Vector3};

use defines::{pipe, ColorFormat, DepthFormat, Locals, Vertex};

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Context {
    event_loop: glutin::EventsLoop,
    window: glutin::Window,
    device: gfx_device_gl::Device,
    pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    data: pipe::Data<gfx_device_gl::Resources>,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    pub factory: gfx_device_gl::Factory
}

impl Context {
    pub fn new() -> Self {
        let event_loop = glutin::EventsLoop::new();
        let builder = build_window();
        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &event_loop);
        let encoder: Encoder<_, _> = factory.create_command_buffer().into();

        let texels = [[0x20, 0xA0, 0xC0, 0x00]];
        let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
            texture::Kind::D2(1, 1, texture::AaMode::Single), &[&texels]
        ).unwrap();

        let pso = factory.create_pipeline_simple(
            include_bytes!("../assets/shaders/plane_150.glslv"),
            include_bytes!("../assets/shaders/plane_150.glslf"),
            pipe::new()
        ).unwrap();

        let sinfo = texture::SamplerInfo::new(
            texture::FilterMethod::Bilinear,
            texture::WrapMode::Clamp);

        let proj = cgmath::perspective(Deg(45.0f32), 1.333, 1.0, 10.0);

        let data = pipe::Data {
            vbuf: factory.create_vertex_buffer(&[]),
            transform: (proj * default_view()).into(),
            locals: factory.create_constant_buffer(1),
            color: (texture_view, factory.create_sampler(sinfo)),
            out_color: main_color,
            out_depth: main_depth
        };

        Context { event_loop, window, device, pso, data, encoder, factory }
    }

    pub fn handle_event<F>(&mut self, mut handler: F) 
        where F: FnMut(WindowEvent)
    {
        let mut events = Vec::new();
        self.event_loop.poll_events(|glutin::Event::WindowEvent { window_id: _, event }| events.push(event));
        
        for e in events {
            match e {
                WindowEvent::Resized(_w, _h) =>
                    gfx_window_glutin::update_views(&self.window, &mut self.data.out_color, &mut self.data.out_depth),
                _ => handler(e)
            }
        }
    }

    pub fn clear(&mut self) {
        self.encoder.clear(&self.data.out_color, CLEAR_COLOR);
        self.encoder.clear_depth(&self.data.out_depth, 1.0);
    }

    pub fn render_mesh(&mut self,
                       locals: &Locals,
                       vertice: &gfx_core::handle::Buffer<gfx_device_gl::Resources, Vertex>,
                       slice: &gfx::Slice<gfx_device_gl::Resources>)
    {
        self.data.vbuf = vertice.clone();
        self.encoder.update_constant_buffer(&self.data.locals, &locals);
        self.encoder.draw(slice, &self.pso, &self.data);
    }

    pub fn flush(&mut self) {
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().unwrap();
        self.device.cleanup();
    }
}

fn build_window() -> glutin::WindowBuilder<'static> {
    glutin::WindowBuilder::new()
        .with_title("Tiangle example".to_string())
        .with_dimensions(1024, 768)
        .with_vsync()
}

fn default_view() -> Matrix4<f32> {
    Matrix4::look_at(
        Point3::new(1.5f32, -5.0, 3.0),
        Point3::new(0.0f32, 0.0, 0.0),
        Vector3::unit_z()
    )
}
