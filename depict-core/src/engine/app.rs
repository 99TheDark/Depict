use std::{cell::RefCell, rc::Rc, sync::Arc};

use wgpu::SurfaceError;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::core::{settings::Settings, system::System};

use super::{properties::Size, state::State};

pub(crate) struct AppData<'a> {
    pub system: Rc<RefCell<dyn System<'a>>>,
    pub settings: Settings,
}

pub(crate) enum App<'a> {
    Uninitialized(AppData<'a>),
    Initialized(State<'a>),
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match self {
            App::Uninitialized(data) => {
                let window = Arc::new(
                    event_loop
                        .create_window(data.settings.attributes().clone())
                        .unwrap(),
                );

                *self = App::Initialized(pollster::block_on(State::new(
                    window.clone(),
                    data.system.clone(),
                    &data.settings,
                )));
            }
            App::Initialized(_) => println!("Resumed"),
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let state = match self {
            App::Uninitialized(_) => return,
            App::Initialized(state) => state,
        };

        match event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                event_loop.exit();
            }
            WindowEvent::Moved(..) => {
                state.reload();
            }
            WindowEvent::Resized(size) => {
                state.resize(Size::from_physical(size));
            }
            WindowEvent::RedrawRequested => {
                match state.render() {
                    Ok(_) => {}
                    Err(SurfaceError::Lost | SurfaceError::Outdated) => state.reload(),
                    Err(e) => eprintln!("{:?}", e),
                }
                state.window.request_redraw();
            }
            _ => {}
        }

        state.mouse.update(&event);
        state.keyboard.update(&event);

        state.update();

        state.mouse.step();
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        match self {
            App::Uninitialized(_) => return,
            App::Initialized(state) => state.window.request_redraw(),
        };
    }
}
