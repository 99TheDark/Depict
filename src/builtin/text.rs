use depict_macro::shape;

use crate::{
    core::{
        properties::{Align, OverflowBreak},
        renderable::Renderable,
    },
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
        // height: Option<f32> = None,
        align: Align = Align::Left,
        color: Color = Color::BLACK,
        overflow_break: OverflowBreak = OverflowBreak::Word,
    }
);

#[derive(Debug, Clone)]
struct Chunk {
    indices: Vec<usize>,
    width: f32,
}

impl Chunk {
    fn new(indices: Vec<usize>, width: f32) -> Self {
        Self { indices, width }
    }

    fn empty() -> Self {
        Self {
            indices: Vec::new(),
            width: 0.0,
        }
    }

    fn reset(&mut self) {
        self.indices.clear();
        self.width = 0.0;
    }

    fn add(&mut self, index: usize, width: f32) {
        self.indices.push(index);
        self.width += width;
    }

    fn extend(&mut self, chunk: &Chunk) {
        self.indices.extend(chunk.indices.clone());
        self.width += chunk.width;
    }

    fn is_empty(&self) -> bool {
        self.indices.is_empty() && self.width == 0.0
    }

    fn flatten(chunks: &Vec<Chunk>) -> Chunk {
        let mut accumulator = Chunk::empty();
        for chunk in chunks {
            accumulator.extend(&chunk);
        }
        accumulator
    }
}

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
        let max_width = if let Some(width) = self.width {
            width
        } else {
            f32::INFINITY
        };

        let data = batch.assets.fonts.data[&self.id].clone();
        let (lines, used_width) = self.lines(&data, max_width);

        let total_width = if let Some(width) = self.width {
            width
        } else {
            used_width
        };

        let vertical_shift = self.size * POINT_TO_PIXELS * self.line_height;

        let mut calc_x = 0.0;
        let mut calc_y = 0.0;
        'outer: for line in lines {
            for idx in line.indices {
                let glyph = data.glyphs[idx];
                let metrics = data.metrics[idx];

                match glyph.character {
                    '\n' => {
                        continue 'outer;
                    }
                    '\r' => {
                        calc_x = 0.0;
                        calc_y -= vertical_shift;
                        continue 'outer;
                    }
                    _ => {}
                }

                let x = self.x + calc_x + metrics.xmin as f32;
                let y = self.y + calc_y - metrics.ymin as f32;

                let width = metrics.width as f32;
                let height = metrics.height as f32;

                let x_offset = match self.align {
                    Align::Left => 0.0,
                    Align::Center => (total_width - line.width) * 0.5,
                    Align::Right => total_width - line.width,
                };

                self.draw_char(x + x_offset, y, width, height, glyph.image_id, batch);

                calc_x += metrics.advance_width;
            }

            calc_x = 0.0;
            calc_y += vertical_shift;
        }
    }
}

impl Text {
    // Chunks of glyph and metrics indicies by line, alongside the bounding width
    fn lines(&self, data: &TextRenderingData, max_width: f32) -> (Vec<Chunk>, f32) {
        let mut glyph_groups = Vec::new();
        let mut cur_group = Vec::new();
        for (i, c) in self.text.chars().enumerate() {
            if c == ' ' || c == '\n' || c == '\r' || c == '\t' {
                glyph_groups.push(cur_group.clone());
                glyph_groups.push(vec![i]);
                cur_group.clear();
                continue;
            }

            cur_group.push(i);
        }
        if !cur_group.is_empty() {
            glyph_groups.push(cur_group);
        }

        let mut lines = Vec::new();
        let mut cur_line = Chunk::empty();
        let mut width = 0.0;
        'outer: for group in glyph_groups {
            let mut breaks = Vec::new();
            let mut cur_break = Chunk::empty();
            for idx in group {
                let glyph = data.glyphs[idx];
                let metrics = data.metrics[idx];

                if glyph.character == '\n' || glyph.character == '\r' {
                    lines.push(cur_line.clone());
                    lines.push(Chunk::new(vec![idx], 0.0));
                    cur_line.reset();
                    continue 'outer;
                }

                let line_width = cur_line.width + cur_break.width;
                if line_width == 0.0 && glyph.character == ' ' {
                    continue;
                }

                if line_width + metrics.width as f32 > max_width {
                    breaks.push(cur_break.clone());
                    cur_break = Chunk::new(vec![idx], metrics.advance_width);
                } else {
                    cur_break.add(idx, metrics.advance_width);

                    if line_width + metrics.advance_width > max_width {
                        breaks.push(cur_break.clone());
                        cur_break.reset();
                    }
                }
            }
            if !cur_break.is_empty() {
                breaks.push(cur_break);
            }

            let glyph_chunk = Chunk::flatten(&breaks);
            if glyph_chunk.width > width {
                width = glyph_chunk.width;
            }

            if glyph_chunk.width > max_width {
                for chunk in breaks {
                    cur_line.extend(&chunk);
                    lines.push(cur_line.clone());
                    cur_line.reset();
                }
            } else if cur_line.width + glyph_chunk.width > max_width {
                lines.push(cur_line.clone());
                cur_line.reset();
                cur_line.extend(&glyph_chunk);
            } else {
                cur_line.extend(&glyph_chunk);
            }
        }
        if !cur_line.is_empty() {
            lines.push(cur_line);
        }

        (lines, width * POINT_TO_PIXELS)
    }

    fn draw_char(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        image_id: u32,
        batch: &mut RenderBatch,
    ) {
        let image = batch.assets.fonts.atlas.get(image_id).clone();

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
    }
}
