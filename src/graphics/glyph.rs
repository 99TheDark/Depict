use fontdue::Metrics;

use super::{
    color::Color,
    font::{FontEmphasis, FontThickness},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Glyph {
    pub character: char,
    pub font_id: u32,
    pub image_id: u32,
    pub size: f32,
    pub color: Color,
    pub thickness: FontThickness,
    pub emphasis: FontEmphasis,
}

#[derive(Debug)]
pub(crate) struct TextRenderingData {
    pub glyphs: Vec<Glyph>,
    pub metrics: Vec<Metrics>,
}
