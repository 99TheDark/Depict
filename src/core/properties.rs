use crate::graphics::{
    asset::{Asset, Image},
    color::Color,
};

#[derive(Debug, Copy, Clone)]
pub enum Background {
    Color(Color),
    Image(Asset<Image>),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Border {
    pub thickness: f32,
    pub color: Color,
}

impl Border {
    pub const NONE: Self = Self {
        thickness: 0.0,
        color: Color::CLEAR,
    };
}
