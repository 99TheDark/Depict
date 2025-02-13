use std::f32::consts::TAU;

use depict_macro::shape;

use crate::{
    core::renderable::Renderable,
    engine::{properties::Properties, renderer::RenderBatch, shader::Vertex},
    graphics::{asset::Assets, color::Color},
};

use super::border::Border;

shape!(
    pub struct Circle {
        x: f32,
        y: f32,
        radius: f32,
        color: Color = Color::CLEAR,
        border: Border = Border::NONE,
    }
);

impl Renderable for Circle {
    fn request(&self, _assets: &mut Assets, _properties: &Properties) {}

    fn render(&self, batch: &mut RenderBatch, _properties: &Properties) {
        const ANGLE_STEP: f32 = TAU / 3.0;

        let apparent_thickness = self.border.apparent_thickness();
        if self.color == Color::CLEAR && apparent_thickness == 0.0 {
            return;
        }

        let approximate_iterations = ((self.radius + apparent_thickness) / 3.0).ln();
        let iterations = u32::max(approximate_iterations.round() as u32, 1);

        let mut points = Vec::with_capacity(3);
        for i in 0..3 {
            points.push((
                self.x + self.radius * (ANGLE_STEP * i as f32).cos(),
                self.y + self.radius * (ANGLE_STEP * i as f32).sin(),
            ));
        }

        batch.triangle(
            Vertex::colored(points[0].0, points[0].1, self.color),
            Vertex::colored(points[1].0, points[1].1, self.color),
            Vertex::colored(points[2].0, points[2].1, self.color),
        );

        // TODO: Optimize significantly, especially the array creation and replacement
        for _ in 0..iterations {
            let mut updated_points = Vec::with_capacity(points.len() * 2);
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

        if apparent_thickness == 0.0 {
            return;
        }

        let mut border_points = Vec::with_capacity(points.len());
        let multiplier = (self.border.thickness + self.radius) / self.radius;
        for point in &points {
            border_points.push((
                (point.0 - self.x) * multiplier + self.x,
                (point.1 - self.y) * multiplier + self.y,
            ));
        }

        for i in 0..points.len() {
            let cur_edge_point = points[i];
            let next_edge_point = points[(i + 1) % points.len()];
            let cur_border_point = border_points[i];
            let next_border_point = border_points[(i + 1) % border_points.len()];

            batch.triangle(
                Vertex::colored(cur_edge_point.0, cur_edge_point.1, self.border.color),
                Vertex::colored(next_edge_point.0, next_edge_point.1, self.border.color),
                Vertex::colored(cur_border_point.0, cur_border_point.1, self.border.color),
            );
            batch.triangle(
                Vertex::colored(next_edge_point.0, next_edge_point.1, self.border.color),
                Vertex::colored(cur_border_point.0, cur_border_point.1, self.border.color),
                Vertex::colored(next_border_point.0, next_border_point.1, self.border.color),
            );
        }
    }
}
