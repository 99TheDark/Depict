use glam::Vec2;
use num::Num;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dimension<T: Num + Copy> {
    pub width: T,
    pub height: T,
}

impl<T: Num + Copy> Dimension<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn square(size: T) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    pub fn to_array(&self) -> [T; 2] {
        [self.width, self.height]
    }
}

impl Dimension<f32> {
    pub fn to_vec(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

impl<T: Num + Copy> std::ops::Mul<T> for Dimension<T> {
    type Output = Dimension<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.width * rhs, self.height * rhs)
    }
}

impl<T: Num + Copy> std::ops::Div<T> for Dimension<T> {
    type Output = Dimension<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.width / rhs, self.height / rhs)
    }
}
