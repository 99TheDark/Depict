use depict_macro::shape;

use crate::{
    core::renderable::Renderable,
    engine::{properties::Properties, renderer::RenderBatch, shader::Vertex},
    graphics::{asset::Assets, color::Color},
};

shape!(
    pub struct Triangle {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        color: Color = Color::CLEAR,
    }
);

impl Renderable for Triangle {
    fn request(&self, _assets: &mut Assets, _properties: &Properties) {}

    fn render(&self, batch: &mut RenderBatch, _properties: &Properties) {
        if self.color == Color::CLEAR {
            return;
        }

        batch.triangle(
            Vertex::colored(self.x1, self.y1, self.color),
            Vertex::colored(self.x2, self.y2, self.color),
            Vertex::colored(self.x3, self.y3, self.color),
        );
    }
}
