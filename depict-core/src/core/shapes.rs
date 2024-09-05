use depict_macro::shape;

use crate::engine::{renderer::RenderBatch, shader::Vertex};

use super::{color::Color, properties::Border, render::Renderable};

shape!(
    pub struct Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        fill: Color = Color::CLEAR, // TODO: This needs to be implemented at vertex and in shader
        border: Border = Border::NONE,
    }
);

impl Renderable for Rectangle {
    fn render(&self, batch: &mut RenderBatch) {
        batch.triangle(
            Vertex {
                pos: [self.x, self.y],
                color: self.fill.to_array(),
                uv: [0.0, 0.0],
                tex_idx: 0,
            },
            Vertex {
                pos: [self.x + self.width, self.y],
                color: self.fill.to_array(),
                uv: [0.0, 0.0],
                tex_idx: 0,
            },
            Vertex {
                pos: [self.x, self.y + self.height],
                color: self.fill.to_array(),
                uv: [0.0, 0.0],
                tex_idx: 0,
            },
        );

        batch.triangle(
            Vertex {
                pos: [self.x + self.width, self.y],
                color: self.fill.to_array(),
                uv: [0.0, 0.0],
                tex_idx: 0,
            },
            Vertex {
                pos: [self.x, self.y + self.height],
                color: self.fill.to_array(),
                uv: [0.0, 0.0],
                tex_idx: 0,
            },
            Vertex {
                pos: [self.x + self.width, self.y + self.height],
                color: self.fill.to_array(),
                uv: [0.0, 0.0],
                tex_idx: 0,
            },
        );
    }
}
