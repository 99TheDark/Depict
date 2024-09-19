use std::f32::consts::TAU;

use depict_macro::shape;

use crate::{
    core::renderable::Renderable,
    engine::{properties::Properties, renderer::RenderBatch, shader::Vertex},
    graphics::{asset::Assets, color::Color},
};

shape!(
    pub struct Circle {
        x: f32,
        y: f32,
        radius: f32,
        color: Color = Color::CLEAR,
    }
);

impl Renderable for Circle {
    fn request(&self, _assets: &mut Assets, _properties: &Properties) {}

    fn render(&self, batch: &mut RenderBatch, _properties: &Properties) {
        if self.color == Color::CLEAR {
            return;
        }

        let segments = u32::max(f32::sqrt(self.radius * 10.0) as u32, 10);
        for i in 0..segments {
            let start_angle = TAU / segments as f32 * i as f32;
            let end_angle = TAU / segments as f32 * (i + 1) as f32;

            batch.triangle(
                Vertex::new(self.x, self.y, 0.0, 0.0, self.color, u32::MAX),
                Vertex::new(
                    self.x + self.radius * start_angle.cos(),
                    self.y + self.radius * start_angle.sin(),
                    0.0,
                    0.0,
                    self.color,
                    u32::MAX,
                ),
                Vertex::new(
                    self.x + self.radius * end_angle.cos(),
                    self.y + self.radius * end_angle.sin(),
                    0.0,
                    0.0,
                    self.color,
                    u32::MAX,
                ),
            );
        }
    }
}
