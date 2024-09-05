use winit::{dpi::Position, window::WindowAttributes};

use crate::engine::properties::Size;

use super::color::Color;

pub struct Settings {
    pub background: Color,
    pub size: Option<Size>,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub position: Option<Position>,
    pub resizable: bool,
    pub title: String,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub active: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: Color::CLEAR,
            size: None,
            min_size: None,
            max_size: None,
            position: None,
            resizable: true,
            title: "".to_string(),
            maximized: false,
            visible: true,
            transparent: false,
            active: true,
        }
    }
}

impl Settings {
    pub fn attributes(&self) -> WindowAttributes {
        let mut attributes = WindowAttributes::default();

        attributes.inner_size = self.size.map(|s| s.physical());
        attributes.min_inner_size = self.min_size.map(|s| s.physical());
        attributes.max_inner_size = self.max_size.map(|s| s.physical());
        attributes.position = self.position;
        attributes.resizable = self.resizable;
        attributes.title = self.title.clone();
        attributes.maximized = self.maximized;
        attributes.visible = self.visible;
        attributes.transparent = self.transparent;
        attributes.active = self.active;

        attributes
    }
}
