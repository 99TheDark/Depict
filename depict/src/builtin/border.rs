use crate::graphics::color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Border {
    pub color: Color,
    pub thickness: f32,
}

impl Border {
    pub const NONE: Self = Self {
        color: Color::CLEAR,
        thickness: 0.0,
    };

    pub fn new(color: Color, thickness: f32) -> Self {
        Self { color, thickness }
    }

    pub fn apparent_thickness(&self) -> f32 {
        if self.color == Color::CLEAR {
            return 0.0;
        } else {
            return self.thickness;
        }
    }
}
