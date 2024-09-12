use crate::graphics::{
    asset::{Asset, Image},
    color::Color,
};

#[derive(Debug, Copy, Clone)]
pub enum Background {
    Color(Color),
    Image(Asset<Image>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OverflowBreak {
    Character,
    Word,
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
