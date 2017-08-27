
use gfx;
use glutin;
use cgmath;
use gfx_device_gl;
use gfx_window_glutin;

use gfx::{Device, Encoder};
use gfx::traits::FactoryExt;
use gfx_core::Factory;
use glutin::{WindowEvent, VirtualKeyCode};

use cgmath::{Matrix4, Deg, SquareMatrix};

use defines::{pipe, ColorFormat, DepthFormat, Vertex};
use texture::Texture;
use mesh::{Object, Mesh};

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Context {
    pub factory: gfx_device_gl::Factory,
    pub projection: Matrix4<f32>,

    event_loop: glutin::EventsLoop,
    window: glutin::Window,
    device: gfx_device_gl::Device,
    pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    data: pipe::Data<gfx_device_gl::Resources>,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,

    running: bool
}

impl Context {
    pub fn new() -> Self {
        let event_loop = glutin::EventsLoop::new();
        let builder = build_window();
        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &event_loop);
        let encoder: Encoder<_, _> = factory.create_command_buffer().into();

        let texels = [0x20, 0xA0, 0xC0, 0x00];
        let texture = Texture::from_raw(&mut factory, &texels);

        let pso = factory.create_pipeline_simple(
            include_bytes!("../assets/shaders/basic_150.glslv"),
            include_bytes!("../assets/shaders/basic_150.glslf"),
            pipe::new()
        ).unwrap();

        let sinfo = gfx::texture::SamplerInfo::new(
            gfx::texture::FilterMethod::Bilinear,
            gfx::texture::WrapMode::Clamp);

        let projection = cgmath::perspective(Deg(45.0f32), 1.333, 1.0, 100.0);

        let data = pipe::Data {
            vbuf: factory.create_vertex_buffer(&[]),
            transform: Matrix4::identity().into(),
            locals: factory.create_constant_buffer(1),
            texture: (texture.view(), factory.create_sampler(sinfo)),
            out_color: main_color,
            out_depth: main_depth
        };

        Context { event_loop, window, device, pso, data, encoder, factory, projection, running: true }
    }

    pub fn is_running(&self) -> bool {
        self.running
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
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Escape), _) | WindowEvent::Closed => self.running = false,
                _ => handler(e)
            }
        }
    }

    pub fn clear(&mut self) {
        self.encoder.clear(&self.data.out_color, CLEAR_COLOR);
        self.encoder.clear_depth(&self.data.out_depth, 1.0);
    }

    pub fn render<M: Object>(&mut self, model: &M) {
        let mesh = model.mesh();
        self.data.vbuf = mesh.vertices().clone();
        self.data.texture.0 = mesh.texture().view();
        self.encoder.update_constant_buffer(&self.data.locals, mesh.locals());
        self.encoder.draw(mesh.slice(), &self.pso, &self.data);
    }

    pub fn flush(&mut self) {
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().unwrap();
        self.device.cleanup();
    }

    pub fn create_texture(&mut self, data: &[u8]) -> Texture {
        Texture::from_raw(&mut self.factory, data)
    }

    pub fn create_mesh(&mut self,
                       position: Matrix4<f32>,
                       cam: Matrix4<f32>,
                       texture: &[u8],
                       vertices: &[Vertex],
                       indexes: &[u32]) -> Mesh {
        Mesh::new(self, position, cam, texture, vertices, indexes)
    }

    pub fn get_viewport_size(&self) -> Option<(u32, u32)> {
        self.window.get_inner_size_pixels()
    }
}

fn build_window() -> glutin::WindowBuilder<'static> {
    glutin::WindowBuilder::new()
        .with_title("Tanks-rs".to_string())
        .with_dimensions(1024, 768)
        .with_vsync()
}
