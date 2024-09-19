use std::collections::{HashMap, HashSet};

use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::PhysicalKey,
};

use super::tracker::Tracker;

#[derive(Debug, Clone)]
pub struct Keyboard {
    keys: HashMap<PhysicalKey, KeyEvent>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: HashMap::new(),
        }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    self.keys.insert(event.physical_key, event.clone());
                } else {
                    self.keys.remove(&event.physical_key);
                }
            }
            _ => {}
        }
    }

    pub fn is_pressed(&self, key: PhysicalKey) -> bool {
        self.keys.contains_key(&key)
    }

    pub fn pressed_keys(&self) -> Vec<&KeyEvent> {
        self.keys.values().collect()
    }
}

impl Tracker<Keyboard> {
    pub fn just_changed(&self) -> (Vec<&KeyEvent>, Vec<&KeyEvent>) {
        let last_keys: HashSet<_> = HashSet::from_iter(self.last.keys.keys());
        let cur_keys: HashSet<_> = HashSet::from_iter(self.keys.keys());

        let forward_diff = &last_keys - &cur_keys;
        let backward_diff = &cur_keys - &last_keys;
        let change = forward_diff.union(&backward_diff);

        let mut added = Vec::new();
        let mut removed = Vec::new();
        for key in change {
            let last_has = last_keys.contains(key);
            let cur_has = cur_keys.contains(key);

            if last_has && !cur_has {
                removed.push(&self.last.keys[key]);
            } else if !last_has && cur_has {
                added.push(&self.keys[key]);
            }
        }

        (added, removed)
    }

    pub fn just_pressed(&self) -> Vec<&KeyEvent> {
        let (pressed, _) = self.just_changed();
        pressed
    }

    pub fn just_released(&self) -> Vec<&KeyEvent> {
        let (_, released) = self.just_changed();
        released
    }
}
