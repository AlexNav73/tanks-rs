
use gfx;
use gfx_core;
use gfx_device_gl;

use gfx::texture;
use gfx_core::Factory;

type ShaderView = gfx_core::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>;

#[derive(Clone)]
pub struct Texture(ShaderView);

impl Texture {
    pub fn from_raw(factory: &mut gfx_device_gl::Factory, data: &[u8]) -> Self {
        let (_, texture_view) = factory.create_texture_immutable_u8::<gfx::format::Rgba8>(
            texture::Kind::D2(1, 1, texture::AaMode::Single), &[data]
        ).unwrap();

        Texture(texture_view)
    }

    pub fn view(&self) -> ShaderView {
        self.0.clone()
    }
}

