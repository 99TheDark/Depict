use depict_macro::shape;

use crate::{
    core::{properties::Background, renderable::Renderable},
    engine::{properties::Properties, renderer::RenderBatch, shader::Vertex},
    graphics::{asset::Assets, color::Color},
};

shape!(
    pub struct Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        background: Background = Background::Color(Color::CLEAR),
        // border: Border = Border::NONE,
    }
);

impl Renderable for Rectangle {
    fn request(&self, _assets: &mut Assets, _properties: &Properties) {}

    fn render(&self, batch: &mut RenderBatch, _properties: &Properties) {
        match self.background {
            Background::Color(color) => {
                batch.triangle(
                    Vertex::new(self.x, self.y, 0.0, 0.0, color, u32::MAX),
                    Vertex::new(self.x + self.width, self.y, 0.0, 0.0, color, u32::MAX),
                    Vertex::new(self.x, self.y + self.height, 0.0, 0.0, color, u32::MAX),
                );
                batch.triangle(
                    Vertex::new(self.x + self.width, self.y, 0.0, 0.0, color, u32::MAX),
                    Vertex::new(self.x, self.y + self.height, 0.0, 0.0, color, u32::MAX),
                    Vertex::new(
                        self.x + self.width,
                        self.y + self.height,
                        0.0,
                        0.0,
                        color,
                        u32::MAX,
                    ),
                );
            }
            Background::Image(asset) => {
                let image = batch.assets.images.get(asset.id).clone();

                batch.triangle(
                    Vertex::new(self.x, self.y, image.u, image.v, Color::CLEAR, 0),
                    Vertex::new(
                        self.x + self.width,
                        self.y,
                        image.u + image.width,
                        image.v,
                        Color::CLEAR,
                        0,
                    ),
                    Vertex::new(
                        self.x,
                        self.y + self.height,
                        image.u,
                        image.v + image.height,
                        Color::CLEAR,
                        0,
                    ),
                );

                batch.triangle(
                    Vertex::new(
                        self.x + self.width,
                        self.y,
                        image.u + image.width,
                        image.v,
                        Color::CLEAR,
                        0,
                    ),
                    Vertex::new(
                        self.x,
                        self.y + self.height,
                        image.u,
                        image.v + image.height,
                        Color::CLEAR,
                        0,
                    ),
                    Vertex::new(
                        self.x + self.width,
                        self.y + self.height,
                        image.u + image.width,
                        image.v + image.height,
                        Color::CLEAR,
                        0,
                    ),
                );
            }
        }
    }
}
