#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl BorderRadius {
    pub const NONE: Self = Self {
        top_left: 0.0,
        top_right: 0.0,
        bottom_left: 0.0,
        bottom_right: 0.0,
    };

    pub fn new(top_left: f32, top_right: f32, bottom_left: f32, bottom_right: f32) -> Self {
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }

    pub fn all(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_left: radius,
            bottom_right: radius,
        }
    }

    pub fn top_left(top_left: f32) -> Self {
        Self {
            top_left,
            top_right: 0.0,
            bottom_left: 0.0,
            bottom_right: 0.0,
        }
    }

    pub fn top_right(top_right: f32) -> Self {
        Self {
            top_left: 0.0,
            top_right,
            bottom_left: 0.0,
            bottom_right: 0.0,
        }
    }

    pub fn bottom_left(bottom_left: f32) -> Self {
        Self {
            top_left: 0.0,
            top_right: 0.0,
            bottom_left,
            bottom_right: 0.0,
        }
    }

    pub fn bottom_right(bottom_right: f32) -> Self {
        Self {
            top_left: 0.0,
            top_right: 0.0,
            bottom_left: 0.0,
            bottom_right,
        }
    }

    pub(crate) fn mapped(
        &self,
        width: f32,
        height: f32,
    ) -> ((f32, f32), (f32, f32), (f32, f32), (f32, f32)) {
        (
            (self.top_left / width, self.top_left / height),
            (self.top_right / width, self.top_right / height),
            (self.bottom_left / width, self.bottom_left / height),
            (self.bottom_right / width, self.bottom_right / height),
        )
    }
}
