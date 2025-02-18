use std::f32::consts::PI;

use depict_macro::shape;

use crate::{
    builtin::border_radius::BorderRadius,
    core::{properties::Background, renderable::Renderable},
    engine::{
        properties::Properties,
        renderer::RenderBatch,
        vertex::Vertex,
        vertex_builder::{UVMappingMode, VertexBuilder, VertexBuildingMode},
    },
    graphics::{asset::Assets, color::Color},
};

use super::{arc::CircularArc, border::Border};

shape!(
    pub struct Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        background: Background = Background::Color(Color::CLEAR),
        border_radius: BorderRadius = BorderRadius::NONE,
        border: Border = Border::NONE,
    }
);

impl Rectangle {
    fn rect(
        &self,
        batch: &mut RenderBatch,
        builder: &VertexBuilder,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) {
        batch.triangle(
            builder.vertex(x1, y1),
            builder.vertex(x2, y1),
            builder.vertex(x1, y2),
        );
        batch.triangle(
            builder.vertex(x2, y1),
            builder.vertex(x1, y2),
            builder.vertex(x2, y2),
        );
    }

    fn render_straight(&self, batch: &mut RenderBatch, builder: VertexBuilder, borderless: bool) {
        self.rect(batch, &builder, 0.0, 0.0, 1.0, 1.0);

        let athick = self.border.apparent_thickness();
        if athick == 0.0 {
            return;
        }

        if borderless {
            return;
        }

        // Top
        batch.triangle(
            Vertex::colored(self.x - athick, self.y - athick, self.border.color),
            Vertex::colored(self.x - athick, self.y, self.border.color),
            Vertex::colored(self.x + self.width + athick, self.y, self.border.color),
        );
        batch.triangle(
            Vertex::colored(self.x - athick, self.y - athick, self.border.color),
            Vertex::colored(
                self.x + self.width + athick,
                self.y - athick,
                self.border.color,
            ),
            Vertex::colored(self.x + self.width + athick, self.y, self.border.color),
        );

        // Bottom
        batch.triangle(
            Vertex::colored(
                self.x - athick,
                self.y + self.height + athick,
                self.border.color,
            ),
            Vertex::colored(self.x - athick, self.y + self.height, self.border.color),
            Vertex::colored(
                self.x + self.width + athick,
                self.y + self.height,
                self.border.color,
            ),
        );
        batch.triangle(
            Vertex::colored(
                self.x - athick,
                self.y + self.height + athick,
                self.border.color,
            ),
            Vertex::colored(
                self.x + self.width + athick,
                self.y + self.height + athick,
                self.border.color,
            ),
            Vertex::colored(
                self.x + self.width + athick,
                self.y + self.height,
                self.border.color,
            ),
        );

        // Left
        batch.triangle(
            Vertex::colored(self.x - athick, self.y, self.border.color),
            Vertex::colored(self.x, self.y, self.border.color),
            Vertex::colored(self.x, self.y + self.height, self.border.color),
        );
        batch.triangle(
            Vertex::colored(self.x - athick, self.y, self.border.color),
            Vertex::colored(self.x - athick, self.y + self.height, self.border.color),
            Vertex::colored(self.x, self.y + self.height, self.border.color),
        );

        // Right
        batch.triangle(
            Vertex::colored(self.x + self.width + athick, self.y, self.border.color),
            Vertex::colored(self.x + self.width, self.y, self.border.color),
            Vertex::colored(self.x + self.width, self.y + self.height, self.border.color),
        );
        batch.triangle(
            Vertex::colored(self.x + self.width + athick, self.y, self.border.color),
            Vertex::colored(
                self.x + self.width + athick,
                self.y + self.height,
                self.border.color,
            ),
            Vertex::colored(self.x + self.width, self.y + self.height, self.border.color),
        );
    }

    fn render_rounded(
        &self,
        batch: &mut RenderBatch,
        builder: VertexBuilder,
        borderless: bool,
        properties: &Properties,
    ) {
        let (
            (top_left_x, top_left_y),
            (top_right_x, top_right_y),
            (bottom_left_x, bottom_left_y),
            (bottom_right_x, bottom_right_y),
        ) = self.border_radius.mapped(self.width, self.height);

        // Top
        self.rect(
            batch,
            &builder,
            top_left_x,
            0.0,
            1.0 - top_right_x,
            f32::max(top_left_y, top_right_y),
        );

        // Bottom
        self.rect(
            batch,
            &builder,
            bottom_left_x,
            1.0,
            1.0 - bottom_right_x,
            1.0 - f32::max(bottom_left_y, bottom_right_y),
        );

        // Left
        self.rect(
            batch,
            &builder,
            0.0,
            top_left_y,
            f32::max(top_left_x, bottom_left_x),
            1.0 - bottom_left_y,
        );

        // Right
        self.rect(
            batch,
            &builder,
            1.0 - f32::max(top_right_x, bottom_right_x),
            top_right_y,
            1.0,
            1.0 - bottom_right_y,
        );

        // Center
        self.rect(
            batch,
            &builder,
            f32::max(top_left_x, bottom_left_x),
            f32::max(top_left_y, top_right_y),
            1.0 - f32::max(top_right_x, bottom_right_x),
            1.0 - f32::max(bottom_left_y, bottom_right_y),
        );

        /*CircularArc::new(
            self.x + self.border_radius.top_left,
            self.y + self.border_radius.top_left,
            self.border_radius.top_left,
            PI,
            PI * 1.5,
        )
        .with_background(self.background)
        .render(batch, properties);*/

        CircularArc::arc(
            batch,
            builder.slice(1.0 - top_right_x, top_right_y, top_right_x, top_right_y),
            1.0,
            PI * 1.5,
            PI * 2.0,
            self.border,
            self.border_radius.top_right,
        );

        /*CircularArc::new(
            self.x + self.border_radius.bottom_left,
            self.y + self.height - self.border_radius.bottom_left,
            self.border_radius.bottom_left,
            PI * 0.5,
            PI,
        )
        .with_background(self.background)
        .render(batch, properties);

        CircularArc::new(
            self.x + self.width - self.border_radius.bottom_right,
            self.y + self.height - self.border_radius.bottom_right,
            self.border_radius.bottom_right,
            0.0,
            PI * 0.5,
        )
        .with_background(self.background)
        .render(batch, properties);*/
    }
}

impl Renderable for Rectangle {
    fn request(&self, _assets: &mut Assets, _properties: &Properties) {}

    fn render(&self, batch: &mut RenderBatch, properties: &Properties) {
        let builder = VertexBuilder::from_background(
            batch,
            self.background,
            UVMappingMode::Corner,
            self.x,
            self.y,
            self.width,
            self.height,
        );

        let borderless = self.border.apparent_thickness() == 0.0;
        if builder.invisible() && borderless {
            return;
        }

        if self.border_radius == BorderRadius::NONE {
            self.render_straight(batch, builder, borderless);
        } else {
            self.render_rounded(batch, builder, borderless, properties);
        }
    }
}
