use depict_macro::shape;

use crate::{
    core::renderable::Renderable,
    engine::renderer::RenderBatch,
    graphics::{
        asset::{Asset, Assets, Font},
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
    fn request(&self, assets: &mut Assets) {
        // Can't last forever as some elements will be removed and then this will break
        let id = assets.fonts.atlas.sources.len() as u32;
        assets.fonts.atlas.sources.push((id, img));
    }

    fn render(&self, batch: &mut RenderBatch) {}
}
