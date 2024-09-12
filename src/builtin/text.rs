use depict_macro::shape;

use crate::{
    core::renderable::Renderable,
    engine::{renderer::RenderBatch, shader::Vertex},
    graphics::{
        asset::{Asset, Assets, Font},
        color::Color,
        font::{FontEmphasis, FontThickness},
        glyph::Glyph,
    },
};

const POINT_TO_PIXELS: f32 = 4.0 / 3.0;

shape!(
    pub struct Text {
        x: f32,
        y: f32,
        text: String,
        font: Asset<Font>,
        thickness: FontThickness = FontThickness::Regular,
        emphasis: FontEmphasis = FontEmphasis::Regular,
        size: f32 = 16.0,
        color: Color = Color::BLACK,
    }
);

impl Renderable for Text {
    fn request(&self, assets: &mut Assets) {
        for character in self.text.chars() {
            assets.fonts.glyphs.push(Glyph {
                character,
                font_id: self.font.id,
                size: self.size,
                color: self.color,
                thickness: self.thickness,
                emphasis: self.emphasis,
            });
        }
    }

    fn render(&self, batch: &mut RenderBatch) {
        // eventually get the right character obviously
        let image = batch.assets.fonts.atlas.get(0).clone();

        let temp_w = self.size * POINT_TO_PIXELS;
        let temp_h = self.size * POINT_TO_PIXELS;

        batch.triangle(
            Vertex::new(self.x, self.y, image.u, image.v, Color::CLEAR, 1),
            Vertex::new(
                self.x + temp_w,
                self.y,
                image.u + image.width,
                image.v,
                Color::CLEAR,
                1,
            ),
            Vertex::new(
                self.x,
                self.y + temp_h,
                image.u,
                image.v + image.height,
                Color::CLEAR,
                1,
            ),
        );

        batch.triangle(
            Vertex::new(
                self.x + temp_w,
                self.y,
                image.u + image.width,
                image.v,
                Color::CLEAR,
                1,
            ),
            Vertex::new(
                self.x,
                self.y + temp_h,
                image.u,
                image.v + image.height,
                Color::CLEAR,
                1,
            ),
            Vertex::new(
                self.x + temp_w,
                self.y + temp_h,
                image.u + image.width,
                image.v + image.height,
                Color::CLEAR,
                1,
            ),
        );
    }
}
