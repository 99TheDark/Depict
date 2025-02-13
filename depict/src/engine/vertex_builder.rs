use crate::{
    core::properties::Background,
    graphics::{color::Color, image::Image},
};

use super::{renderer::RenderBatch, vertex::Vertex};

#[derive(Debug)]
pub enum VertexBuildingMode {
    Colored(Color),
    Textured(Image),
}

#[derive(Debug, Copy, Clone)]
pub enum UVMappingMode {
    Corner,
    Center,
}

pub struct VertexBuilder {
    pub vertex_mode: VertexBuildingMode,
    pub uv_mode: UVMappingMode,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl VertexBuilder {
    pub fn new(
        vertex_mode: VertexBuildingMode,
        uv_mode: UVMappingMode,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            vertex_mode,
            uv_mode,
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_background(
        batch: &RenderBatch,
        background: Background,
        uv_mode: UVMappingMode,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Self {
        match background {
            Background::Color(color) => VertexBuilder::new(
                VertexBuildingMode::Colored(color),
                uv_mode,
                x,
                y,
                width,
                height,
            ),
            Background::Image(asset) => {
                let image = batch.assets.images.get(asset.id).clone();
                VertexBuilder::new(
                    VertexBuildingMode::Textured(image),
                    uv_mode,
                    x,
                    y,
                    width,
                    height,
                )
            }
        }
    }

    pub fn vertex(&self, x: f32, y: f32) -> Vertex {
        match (&self.vertex_mode, &self.uv_mode) {
            (VertexBuildingMode::Colored(color), ..) => {
                Vertex::colored(self.x + x * self.width, self.y + y * self.height, *color)
            }
            (VertexBuildingMode::Textured(image), UVMappingMode::Corner) => Vertex::textured(
                self.x + x * self.width,
                self.y + y * self.height,
                image.u + x * image.width,
                image.v + y * image.height,
                0, // TODO: Fix, it won't always be 0
            ),
            (VertexBuildingMode::Textured(image), UVMappingMode::Center) => Vertex::textured(
                self.x + x * self.width,
                self.y + y * self.height,
                image.u + (x + 1.0) * 0.5 * image.width,
                image.v + (y + 1.0) * 0.5 * image.height,
                0,
            ),
        }
    }

    pub fn invisible(&self) -> bool {
        match self.vertex_mode {
            VertexBuildingMode::Colored(color) => color == Color::CLEAR,
            VertexBuildingMode::Textured(image) => image.id == u32::MAX,
        }
    }
}
