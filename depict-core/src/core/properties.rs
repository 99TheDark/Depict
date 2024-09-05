use super::color::Color;

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
