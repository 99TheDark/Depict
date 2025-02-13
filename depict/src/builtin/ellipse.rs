use std::f32::consts::TAU;

use depict_macro::shape;

use crate::{
    core::renderable::Renderable,
    engine::{properties::Properties, renderer::RenderBatch, shader::Vertex},
    graphics::{asset::Assets, color::Color},
};

shape!(
    pub struct Ellipse {
        x: f32,
        y: f32,
        x_radius: f32,
        y_radius: f32,
        color: Color = Color::CLEAR,
    }
);

impl Renderable for Ellipse {
    fn request(&self, _assets: &mut Assets, _properties: &Properties) {}

    fn render(&self, batch: &mut RenderBatch, _properties: &Properties) {
        if self.color == Color::CLEAR {
            return;
        }

        let seg_x = f32::sqrt(self.x_radius * 10.0);
        let seg_y = f32::sqrt(self.y_radius * 10.0);
        let segments = u32::max(f32::sqrt(seg_x * seg_y) as u32, 10);
        for i in 0..segments {
            let start_angle = TAU / segments as f32 * i as f32;
            let end_angle = TAU / segments as f32 * (i + 1) as f32;

            batch.triangle(
                Vertex::colored(self.x, self.y, self.color),
                Vertex::colored(
                    self.x + self.x_radius * start_angle.cos(),
                    self.y + self.y_radius * start_angle.sin(),
                    self.color,
                ),
                Vertex::colored(
                    self.x + self.x_radius * end_angle.cos(),
                    self.y + self.y_radius * end_angle.sin(),
                    self.color,
                ),
            );
        }
    }
}
