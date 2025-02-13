use crate::graphics::{color::Color, image::Image};

use super::vertex::Vertex;

pub enum VertexBuildingMode {
    Colored(Color),
    Textured(Image),
}

pub struct VertexBuilder {
    pub mode: VertexBuildingMode,
}

impl VertexBuilder {
    pub fn new(mode: VertexBuildingMode) -> Self {
        Self { mode }
    }

    /// Creates a vertex depending on the mode provided
    ///
    /// # Arguments
    /// * `x` - The x position of the vertex
    /// * `y` - The y position of the vertex
    /// * `u` - The relative 0 to 1 x coordinate of the texture
    /// * `v` - The relative 0 to 1 y coordinate of the texture
    pub fn vertex(&self, x: f32, y: f32, u: f32, v: f32) -> Vertex {
        match self.mode {
            VertexBuildingMode::Colored(color) => Vertex::colored(x, y, color),
            VertexBuildingMode::Textured(image) => Vertex::textured(
                x,
                y,
                image.u + u * image.width,
                image.v + v * image.height,
                0, // TODO: Fix, it won't always be 0
            ),
        }
    }
}
