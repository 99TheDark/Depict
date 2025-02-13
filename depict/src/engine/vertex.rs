use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use wgpu::{BufferAddress, VertexBufferLayout, VertexFormat, VertexStepMode};

use crate::graphics::color::Color;

use super::attributes::Attributes;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable, PartialEq)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
    pub uv: [f32; 2],
    pub atlas_idx: u32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, u: f32, v: f32, color: Color, atlas_idx: u32) -> Self {
        Self {
            pos: [x, y],
            color: color.to_array(),
            uv: [u, v],
            atlas_idx,
        }
    }

    pub fn colored(x: f32, y: f32, color: Color) -> Self {
        Self {
            pos: [x, y],
            color: color.to_array(),
            uv: [0.0, 0.0],
            atlas_idx: u32::MAX,
        }
    }

    pub fn textured(x: f32, y: f32, u: f32, v: f32, atlas_idx: u32) -> Self {
        Self {
            pos: [x, y],
            color: [0.0, 0.0, 0.0, 0.0],
            uv: [u, v],
            atlas_idx,
        }
    }

    pub(crate) fn description() -> VertexBufferLayout<'static> {
        let mut attributes = Attributes::new();
        attributes.add(VertexFormat::Float32x2); // Position
        attributes.add(VertexFormat::Float32x4); // Color
        attributes.add(VertexFormat::Float32x2); // UV
        attributes.add(VertexFormat::Uint32); // Texture ID

        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: attributes.attributes.leak(),
        }
    }
}
