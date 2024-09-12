use depict_macro::shape;

use crate::{
    core::renderable::Renderable,
    engine::{renderer::RenderBatch, shader::Vertex},
    graphics::{
        asset::{Asset, Assets, Font},
        color::Color,
        font::{FontEmphasis, FontThickness},
        glyph::{Glyph, TextRenderingData},
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
        let mut glyphs = Vec::new();
        for character in self.text.chars() {
            glyphs.push(Glyph {
                character,
                font_id: self.font.id,
                image_id: u32::MAX,
                size: self.size,
                color: self.color,
                thickness: self.thickness,
                emphasis: self.emphasis,
            });
        }

        assets.fonts.data.insert(
            self.id,
            TextRenderingData {
                glyphs,
                metrics: Vec::new(),
            },
        );
    }

    fn render(&self, batch: &mut RenderBatch) {
        let data = &batch.assets.fonts.data[&self.id];
        if data.glyphs.len() != data.metrics.len() {
            panic!("Incomplete corresponding metrics to glyphs in text rendering");
        }

        for i in 0..data.glyphs.len() {
            // I really with I could just &data.(something)
            let glyph = &batch.assets.fonts.data[&self.id].glyphs[i];
            let metrics = &batch.assets.fonts.data[&self.id].metrics[i];

            let image = batch.assets.fonts.atlas.get(glyph.image_id).clone();

            let temp_x = self.x + i as f32 * self.size * POINT_TO_PIXELS;
            let temp_y = 0.0;

            let temp_w = self.size * POINT_TO_PIXELS;
            let temp_h = self.size * POINT_TO_PIXELS;

            batch.triangle(
                Vertex::new(temp_x, temp_y, image.u, image.v, Color::CLEAR, 1),
                Vertex::new(
                    temp_x + temp_w,
                    temp_y,
                    image.u + image.width,
                    image.v,
                    Color::CLEAR,
                    1,
                ),
                Vertex::new(
                    temp_x,
                    temp_y + temp_h,
                    image.u,
                    image.v + image.height,
                    Color::CLEAR,
                    1,
                ),
            );

            batch.triangle(
                Vertex::new(
                    temp_x + temp_w,
                    temp_y,
                    image.u + image.width,
                    image.v,
                    Color::CLEAR,
                    1,
                ),
                Vertex::new(
                    temp_x,
                    temp_y + temp_h,
                    image.u,
                    image.v + image.height,
                    Color::CLEAR,
                    1,
                ),
                Vertex::new(
                    temp_x + temp_w,
                    temp_y + temp_h,
                    image.u + image.width,
                    image.v + image.height,
                    Color::CLEAR,
                    1,
                ),
            );
        }
    }
}
