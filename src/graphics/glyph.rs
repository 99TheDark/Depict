use super::{
    color::Color,
    font::{FontEmphasis, FontThickness},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Glyph {
    pub character: char,
    pub font_id: u32,
    pub size: f32,
    pub color: Color,
    pub thickness: FontThickness,
    pub emphasis: FontEmphasis,
}
