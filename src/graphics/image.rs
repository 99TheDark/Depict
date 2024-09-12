#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Image {
    pub(crate) id: u32,
    pub u: f32,
    pub v: f32,
    pub width: f32,
    pub height: f32,
}

impl Image {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            id: u32::MAX,
            u: 0.0,
            v: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}
