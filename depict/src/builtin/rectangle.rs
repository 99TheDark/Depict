use depict_macro::shape;

use crate::{
    core::{properties::Background, renderable::Renderable},
    engine::{properties::Properties, renderer::RenderBatch, shader::Vertex},
    graphics::{asset::Assets, color::Color},
};

use super::border::Border;

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
    fn request(&self, _assets: &mut Assets, _properties: &Properties) {}

    fn render(&self, batch: &mut RenderBatch, _properties: &Properties) {
        match self.background {
            Background::Color(color) => {
                if color == Color::CLEAR {
                    return;
                }

                batch.triangle(
                    Vertex::colored(self.x, self.y, color),
                    Vertex::colored(self.x + self.width, self.y, color),
                    Vertex::colored(self.x, self.y + self.height, color),
                );
                batch.triangle(
                    Vertex::colored(self.x + self.width, self.y, color),
                    Vertex::colored(self.x, self.y + self.height, color),
                    Vertex::colored(self.x + self.width, self.y + self.height, color),
                );
            }
            Background::Image(asset) => {
                let image = batch.assets.images.get(asset.id).clone();

                batch.triangle(
                    Vertex::textured(self.x, self.y, image.u, image.v, 0),
                    Vertex::textured(
                        self.x + self.width,
                        self.y,
                        image.u + image.width,
                        image.v,
                        0,
                    ),
                    Vertex::textured(
                        self.x,
                        self.y + self.height,
                        image.u,
                        image.v + image.height,
                        0,
                    ),
                );

                batch.triangle(
                    Vertex::textured(
                        self.x + self.width,
                        self.y,
                        image.u + image.width,
                        image.v,
                        0,
                    ),
                    Vertex::textured(
                        self.x,
                        self.y + self.height,
                        image.u,
                        image.v + image.height,
                        0,
                    ),
                    Vertex::textured(
                        self.x + self.width,
                        self.y + self.height,
                        image.u + image.width,
                        image.v + image.height,
                        0,
                    ),
                );
            }
        }
    }
}
