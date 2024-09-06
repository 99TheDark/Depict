#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Asset {
    pub(crate) id: u32,
}

impl Asset {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn loaded(&self) -> bool {
        self.id != 0
    }
}
