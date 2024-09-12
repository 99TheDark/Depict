use depict_macro::shape;

use crate::{
    core::{properties::Align, renderable::Renderable},
    engine::{renderer::RenderBatch, shader::Vertex},
    graphics::{
        asset::{Asset, Assets, Font},
        color::Color,
        font::{FontEmphasis, FontThickness},
        glyph::{Glyph, TextRenderingData},
    },
};

pub(crate) const POINT_TO_PIXELS: f32 = 4.0 / 3.0;

shape!(
    pub struct Text {
        x: f32,
        y: f32,
        text: String,
        font: Asset<Font>,
        thickness: FontThickness = FontThickness::Regular,
        emphasis: FontEmphasis = FontEmphasis::Regular,
        size: f32 = 16.0,
        line_height: f32 = 1.2,
        width: Option<f32> = None,
        height: Option<f32> = None,
        align: Align = Align::Left,
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

    // TODO: Add support for vertical fonts
    fn render(&self, batch: &mut RenderBatch) {
        let data = batch.assets.fonts.data[&self.id].clone();
        if data.glyphs.len() != data.metrics.len() {
            panic!("Incomplete corresponding metrics to glyphs in text rendering");
        }

        let mut cur_x = self.x;
        let mut cur_y = self.y;
        for i in 0..data.glyphs.len() {
            let glyph = data.glyphs[i];
            let metrics = data.metrics[i];

            match glyph.character {
                '\n' => {
                    cur_x = self.x;
                    cur_y += self.size * POINT_TO_PIXELS * self.line_height;
                    continue;
                }
                '\r' => {
                    cur_x = self.x;
                    continue;
                }
                _ => {}
            }

            let image = batch.assets.fonts.atlas.get(glyph.image_id).clone();

            let x = cur_x + metrics.xmin as f32;
            let y = cur_y - metrics.ymin as f32;

            let width = metrics.width as f32;
            let height = metrics.height as f32;

            batch.triangle(
                Vertex::new(x, y, image.u, image.v + image.height, Color::CLEAR, 1),
                Vertex::new(
                    x + width,
                    y,
                    image.u + image.width,
                    image.v + image.height,
                    Color::CLEAR,
                    1,
                ),
                Vertex::new(x, y - height, image.u, image.v, Color::CLEAR, 1),
            );

            batch.triangle(
                Vertex::new(
                    x + width,
                    y,
                    image.u + image.width,
                    image.v + image.height,
                    Color::CLEAR,
                    1,
                ),
                Vertex::new(x, y - height, image.u, image.v, Color::CLEAR, 1),
                Vertex::new(
                    x + width,
                    y - height,
                    image.u + image.width,
                    image.v,
                    Color::CLEAR,
                    1,
                ),
            );

            cur_x += metrics.advance_width;
        }
    }
}
