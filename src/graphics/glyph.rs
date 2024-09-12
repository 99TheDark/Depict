use fontdue::Metrics;

use super::{
    color::Color,
    font::{FontEmphasis, FontThickness},
};

#[derive(Debug, Copy, Clone)]
pub(crate) struct Glyph {
    pub character: char,
    pub font_id: u32,
    pub image_id: u32,
    pub size: f32,
    pub color: Color,
    pub thickness: FontThickness,
    pub emphasis: FontEmphasis,
}

impl PartialEq for Glyph {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character
            && self.font_id == other.font_id
            && self.size == other.size
            && self.color == other.color
            && self.thickness == other.thickness
            && self.emphasis == other.emphasis
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TextRenderingData {
    pub glyphs: Vec<Glyph>,
    pub metrics: Vec<Metrics>,
}
