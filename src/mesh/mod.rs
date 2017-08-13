
use gfx;
use gfx::traits::FactoryExt;
use gfx_device_gl::Resources;
use gfx_core::handle::Buffer;

use cgmath::Matrix4;

use texture::Texture;
use defines::{Vertex, Locals};
use context::Context;

pub mod cube;
pub mod model;

pub struct Mesh {
    vertices: Buffer<Resources, Vertex>,
    slice: gfx::Slice<Resources>,
    locals: Locals,
    texture: Texture
}

impl Mesh {
    pub fn new(context: &mut Context,
               matrix: Matrix4<f32>,
               view: Matrix4<f32>,
               texture: &[u8],
               vertex_data: &[Vertex],
               index_data: &[u32]) -> Self {
        let (vertex_buffer, slice) = context.factory.create_vertex_buffer_with_slice(vertex_data, index_data);
        Mesh {
            vertices: vertex_buffer,
            slice,
            locals: Locals {
                transform: matrix.into(),
                view: view.into(),
                proj: context.projection.into()
            },
            texture: context.create_texture(texture)
        }
    }

    pub fn vertices(&self) -> &Buffer<Resources, Vertex> {
        &self.vertices
    }

    pub fn slice(&self) -> &gfx::Slice<Resources> {
        &self.slice
    }

    pub fn locals(&self) -> &Locals {
        &self.locals
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn update(&mut self, ctx: &Context, new: Matrix4<f32>, cam: Matrix4<f32>) {
        self.locals = Locals {
            transform: new.into(),
            view: cam.into(),
            proj: ctx.projection.into()
        }
    }
}

pub trait Object {
    fn mesh(&self) -> &Mesh;
    fn mesh_mut(&mut self) -> &mut Mesh;

    fn transform(&mut self, ctx: &Context, new: Matrix4<f32>, cam: Matrix4<f32>) {
        self.mesh_mut().update(ctx, new, cam);
    }
}
