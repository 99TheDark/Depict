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
        resolution: f32 = 10.0,
    }
);

impl Renderable for Circle {
    fn request(&self, _assets: &mut Assets, _properties: &Properties) {}

    fn render(&self, batch: &mut RenderBatch, _properties: &Properties) {
        if self.color == Color::CLEAR {
            return;
        }

        let segments = u32::max(
            (f32::powf(self.radius, 0.25) * self.resolution * 0.125) as u32,
            3,
        );

        let mut points = Vec::new();
        for i in 0..3 {
            points.push((
                self.x + self.radius * (TAU / 3.0 * i as f32).sin(),
                self.y + self.radius * (TAU / 3.0 * i as f32).cos(),
            ));
        }

        batch.triangle(
            Vertex::colored(points[0].0, points[0].1, self.color),
            Vertex::colored(points[1].0, points[1].1, self.color),
            Vertex::colored(points[2].0, points[2].1, self.color),
        );

        // TODO: Optimize significantly, especially the array creation and replacement
        for _ in 0..segments {
            let mut updated_points = Vec::new();
            for j in 0..points.len() {
                let cur_point = points[j];
                let next_point = points[(j + 1) % points.len()];

                let mid_x = (cur_point.0 + next_point.0) * 0.5;
                let mid_y = (cur_point.1 + next_point.1) * 0.5;

                let dx = mid_x - self.x;
                let dy = mid_y - self.y;

                let mag = (dx * dx + dy * dy).sqrt();

                let new_point = (
                    dx / mag * self.radius + self.x,
                    dy / mag * self.radius + self.y,
                );

                updated_points.push(cur_point);
                updated_points.push(new_point);

                batch.triangle(
                    Vertex::colored(cur_point.0, cur_point.1, self.color),
                    Vertex::colored(next_point.0, next_point.1, self.color),
                    Vertex::colored(new_point.0, new_point.1, self.color),
                );
            }

            points = updated_points;
        }
    }
}
