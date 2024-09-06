use winit::{dpi::Position, window::WindowAttributes};

use crate::engine::properties::Size;

use super::color::Color;

pub struct Settings {
    pub background: Color,
    pub size: Option<Size>,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub position: Option<Position>,
    pub title: String,
    pub resizable: bool,
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
            title: "".to_string(),
            resizable: true,
            maximized: false,
            visible: true,
            transparent: false,
            active: true,
        }
    }
}

impl Settings {
    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_min_size(mut self, min_size: Size) -> Self {
        self.min_size = Some(min_size);
        self
    }

    pub fn with_max_size(mut self, max_size: Size) -> Self {
        self.max_size = Some(max_size);
        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_resizable(mut self, resizeable: bool) -> Self {
        self.resizable = resizeable;
        self
    }

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
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
