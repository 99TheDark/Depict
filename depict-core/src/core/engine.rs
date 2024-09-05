use std::{cell::RefCell, rc::Rc};

use winit::event_loop::EventLoop;

use crate::engine::app::{App, AppData};

use super::{settings::Settings, system::System};

pub struct Engine<'a> {
    app: App<'a>,
}

impl<'a> Engine<'a> {
    pub fn new(settings: Settings, system: Rc<RefCell<dyn System<'a>>>) -> Self {
        Self {
            app: App::Uninitialized(AppData { settings, system }),
        }
    }

    pub fn run(&mut self) {
        env_logger::init();

        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut self.app).unwrap();
    }
}
