use glam::Vec2;
use winit::dpi::PhysicalSize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }

    pub(crate) fn from_physical(size: PhysicalSize<u32>) -> Self {
        Size {
            width: size.width,
            height: size.height,
        }
    }

    pub fn to_vec(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }

    pub(crate) fn physical(&self) -> winit::dpi::Size {
        winit::dpi::Size::Physical(PhysicalSize {
            width: self.width as u32,
            height: self.height as u32,
        })
    }

    pub fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

// TODO: Remove
#[derive(Debug, Copy, Clone)]
pub struct Properties {
    pub size: Size,
    pub aspect: f32,
}
