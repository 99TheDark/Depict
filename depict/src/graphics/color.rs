use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    pub const RED: Self = Self {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
    };
    pub const GREEN: Self = Self {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
        alpha: 1.0,
    };
    pub const BLUE: Self = Self {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
        alpha: 1.0,
    };
    pub const CYAN: Self = Self {
        red: 0.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0,
    };
    pub const MAGENTA: Self = Self {
        red: 1.0,
        green: 0.0,
        blue: 1.0,
        alpha: 1.0,
    };
    pub const YELLOW: Self = Self {
        red: 1.0,
        green: 1.0,
        blue: 0.0,
        alpha: 1.0,
    };
    pub const BLACK: Self = Self {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
    };
    pub const WHITE: Self = Self {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0,
    };
    pub const CLEAR: Self = Self {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 0.0,
    };

    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red: red as f32 / 255.0,
            green: green as f32 / 255.0,
            blue: blue as f32 / 255.0,
            alpha: alpha as f32 / 255.0,
        }
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::from_rgba(red, green, blue, 255)
    }

    pub fn from_hex(hex: u32) -> Self {
        Color {
            red: ((hex >> 24) & 255) as f32 / 255.0,
            green: ((hex >> 16) & 255) as f32 / 255.0,
            blue: ((hex >> 8) & 255) as f32 / 255.0,
            alpha: (hex & 255) as f32 / 255.0,
        }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.red * 255.0) as u32) << 24
            | ((self.green * 255.0) as u32) << 16
            | ((self.blue * 255.0) as u32) << 8
            | ((self.alpha * 255.0) as u32)
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_hex().hash(state);
    }
}
