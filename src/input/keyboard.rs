use std::collections::HashSet;

use winit::{event::WindowEvent, keyboard::PhysicalKey};

#[derive(Debug, Clone)]
pub struct Keyboard {
    keys: HashSet<PhysicalKey>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: HashSet::new(),
        }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() && !event.repeat {
                    self.keys.insert(event.physical_key);
                } else {
                    self.keys.remove(&event.physical_key);
                }
            }
            _ => {}
        }
    }

    pub fn pressed(&self, key: PhysicalKey) -> bool {
        self.keys.contains(&key)
    }
}
