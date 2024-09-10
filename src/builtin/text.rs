use depict_macro::shape;

use crate::{
    core::renderable::Renderable,
    engine::renderer::RenderBatch,
    graphics::{
        asset::{Asset, Font},
        color::Color,
    },
};

use super::rectangle::Rectangle;

shape!(
    pub struct Text {
        x: f32,
        y: f32,
        text: String,
        font: Asset<Font>,
        size: f32 = 16.0,
        color: Color = Color::BLACK,
    }
);

impl Renderable for Text {
    fn render(&self, batch: &mut RenderBatch) {}
}
