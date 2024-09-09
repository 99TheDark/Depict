use crate::graphics::{
    asset::{self, Asset},
    color::Color,
    image::Image,
};

#[derive(Debug, Copy, Clone)]
pub enum Background {
    Color(Color),
    Image(Asset<asset::Image>),
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
