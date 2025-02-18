use depict_macro::shape;

use crate::{
    core::{properties::Background, renderable::Renderable},
    engine::{
        properties::Properties,
        renderer::RenderBatch,
        vertex::Vertex,
        vertex_builder::{UVMappingMode, VertexBuilder},
    },
    graphics::{asset::Assets, color::Color},
};

use super::border::Border;

shape!(
    pub struct CircularArc {
        x: f32,
        y: f32,
        radius: f32, // TODO: Add x_radius, y_radius once ellipses are figured out
        start: f32,
        stop: f32,
        background: Background = Background::Color(Color::CLEAR),
        border: Border = Border::NONE,
    }
);

impl CircularArc {
    pub(crate) fn arc(
        batch: &mut RenderBatch,
        builder: VertexBuilder,
        radius: f32,
        start: f32,
        stop: f32,
        border: Border,
        true_radius: f32,
    ) {
        let apparent_thickness = border.apparent_thickness();
        if builder.invisible() && apparent_thickness == 0.0 {
            return;
        }

        let approximate_iterations = ((true_radius + apparent_thickness) / 3.0).ln();
        let iterations = u32::max(approximate_iterations.round() as u32, 1) + 1;

        let mut points = Vec::with_capacity(3);
        let angle_step = (stop - start) / 2.0;
        for i in 0..3 {
            let angle = start + angle_step * i as f32;
            points.push((angle.cos(), angle.sin()));
        }

        batch.triangle(
            builder.vertex(points[0].0, points[0].1),
            builder.vertex(points[1].0, points[1].1),
            builder.vertex(points[2].0, points[2].1),
        );

        // TODO: Optimize significantly, especially the array creation and replacement
        for _ in 0..iterations {
            let mut updated_points = Vec::with_capacity(points.len() * 2 - 1);
            for j in 0..points.len() - 1 {
                let cur_point = points[j];
                let next_point = points[j + 1];

                let mid_x = (cur_point.0 + next_point.0) * 0.5;
                let mid_y = (cur_point.1 + next_point.1) * 0.5;

                let mag = (mid_x * mid_x + mid_y * mid_y).sqrt();

                let new_point = (mid_x / mag, mid_y / mag);

                updated_points.push(cur_point);
                updated_points.push(new_point);

                batch.triangle(
                    builder.vertex(cur_point.0, cur_point.1),
                    builder.vertex(next_point.0, next_point.1),
                    builder.vertex(new_point.0, new_point.1),
                );
            }

            updated_points.push(*points.last().unwrap());
            points = updated_points;
        }

        let first = points[0];
        let last = points.last().unwrap();
        batch.triangle(
            builder.vertex(0.0, 0.0),
            builder.vertex(first.0, first.1),
            builder.vertex(last.0, last.1),
        );

        if apparent_thickness == 0.0 {
            return;
        }

        let mut border_points = Vec::with_capacity(points.len());
        let multiplier = border.thickness + radius;
        for point in &points {
            border_points.push((
                point.0 * multiplier + builder.x,
                point.1 * multiplier + builder.y,
            ));
        }

        for point in &mut points {
            point.0 = point.0 * radius + builder.x;
            point.1 = point.1 * radius + builder.y;
        }

        for i in 0..points.len() - 1 {
            let cur_edge_point = points[i];
            let next_edge_point = points[i + 1];
            let cur_border_point = border_points[i];
            let next_border_point = border_points[i + 1];

            batch.triangle(
                Vertex::colored(cur_edge_point.0, cur_edge_point.1, border.color),
                Vertex::colored(next_edge_point.0, next_edge_point.1, border.color),
                Vertex::colored(cur_border_point.0, cur_border_point.1, border.color),
            );
            batch.triangle(
                Vertex::colored(next_edge_point.0, next_edge_point.1, border.color),
                Vertex::colored(cur_border_point.0, cur_border_point.1, border.color),
                Vertex::colored(next_border_point.0, next_border_point.1, border.color),
            );
        }
    }
}

impl Renderable for CircularArc {
    fn request(&self, _assets: &mut Assets, _properties: &Properties) {}

    fn render(&self, batch: &mut RenderBatch, _properties: &Properties) {
        let builder = VertexBuilder::from_background(
            batch,
            self.background,
            UVMappingMode::Center,
            self.x,
            self.y,
            self.radius,
            self.radius,
        );

        Self::arc(
            batch,
            builder,
            self.radius,
            self.start,
            self.stop,
            self.border,
            self.radius,
        );
    }
}
