use depict_macro::shape;

use crate::engine::{renderer::RenderBatch, shader::Vertex};

use super::{
    color::Color,
    properties::{Background, Border},
    render::Renderable,
};

shape!(
    pub struct Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        background: Background = Background::Color(Color::CLEAR),
        border: Border = Border::NONE,
    }
);

impl Renderable for Rectangle {
    fn render(&self, batch: &mut RenderBatch) {
        batch.triangle(
            Vertex::new(self.x, self.y, 0.0, 1.0, &self.background),
            Vertex::new(self.x + self.width, self.y, 1.0, 1.0, &self.background),
            Vertex::new(self.x, self.y + self.height, 0.0, 0.0, &self.background),
        );

        batch.triangle(
            Vertex::new(self.x + self.width, self.y, 1.0, 1.0, &self.background),
            Vertex::new(self.x, self.y + self.height, 0.0, 0.0, &self.background),
            Vertex::new(
                self.x + self.width,
                self.y + self.height,
                1.0,
                0.0,
                &self.background,
            ),
        );
    }
}
