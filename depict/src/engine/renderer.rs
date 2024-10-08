use bytemuck::cast_slice;
use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

use crate::graphics::asset::Assets;

use super::shader::Vertex;

#[derive(Debug)]
pub struct RenderBatch<'a> {
    pub deduplicate: bool,
    pub(crate) assets: &'a mut Assets,
    renderer: &'a mut Renderer,
    vertices: Vec<Vertex>,
    pub(crate) lower_bound: Vec2,
    pub(crate) upper_bound: Vec2,
}

impl<'a> RenderBatch<'a> {
    pub fn triangle(&mut self, a: Vertex, b: Vertex, c: Vertex) {
        let lower_x = f32::min(
            f32::min(a.pos[0], b.pos[0]),
            f32::min(c.pos[0], self.lower_bound.x),
        );
        let lower_y = f32::min(
            f32::min(a.pos[1], b.pos[1]),
            f32::min(c.pos[1], self.lower_bound.y),
        );
        let upper_x = f32::max(
            f32::max(a.pos[0], b.pos[0]),
            f32::max(c.pos[0], self.upper_bound.x),
        );
        let upper_y = f32::max(
            f32::max(a.pos[1], b.pos[1]),
            f32::max(c.pos[1], self.upper_bound.y),
        );

        self.lower_bound.x = lower_x;
        self.lower_bound.y = lower_y;
        self.upper_bound.x = upper_x;
        self.upper_bound.y = upper_y;

        self.vertices.extend([a, b, c]);
    }

    pub fn finish(&mut self) {
        for vertex in &self.vertices {
            if self.deduplicate {
                if let Some(idx) = self.renderer.vertices.iter().position(|v| v == vertex) {
                    self.renderer.indices.push(idx as u16);
                    continue;
                }
            }

            self.renderer.vertices.push(*vertex);
            self.renderer.indices.push(self.renderer.count as u16);
            self.renderer.count += 1;
        }
    }
}

#[derive(Debug)]
pub struct Renderer {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    count: u32,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            vertices: Vec::new(),
            indices: Vec::new(),
            count: 0,
        }
    }

    pub fn batch<'a>(&'a mut self, assets: &'a mut Assets, deduplicate: bool) -> RenderBatch {
        RenderBatch {
            deduplicate,
            assets,
            renderer: self,
            vertices: Vec::new(),
            lower_bound: Vec2::ZERO,
            upper_bound: Vec2::ZERO,
        }
    }

    pub fn build(&mut self, device: &Device) -> (u32, Buffer, Buffer) {
        let vertex_buffer_descriptor = BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&self.vertices),
            usage: BufferUsages::VERTEX,
        };
        let vertex_buffer = device.create_buffer_init(&vertex_buffer_descriptor);

        let index_buffer_descriptor = BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: cast_slice(&self.indices),
            usage: BufferUsages::INDEX,
        };
        let index_buffer = device.create_buffer_init(&index_buffer_descriptor);

        return (self.indices.len() as u32, vertex_buffer, index_buffer);
    }
}
