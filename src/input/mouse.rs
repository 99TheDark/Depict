use glam::Vec2;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
};

use crate::{component::screen::factors, engine::size::Size};

#[derive(Debug, Copy, Clone)]
pub struct Mouse {
    pub inside: bool,
    pub pos: Vec2,
    pub state: ElementState,
    pub button: MouseButton,
    pub delta: MouseScrollDelta,
    pub phase: TouchPhase,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            inside: false,
            pos: Vec2::new(0.0, 0.0),
            state: ElementState::Released,
            button: MouseButton::Left,
            delta: MouseScrollDelta::PixelDelta(PhysicalPosition::new(0.0, 0.0)),
            phase: TouchPhase::Ended,
        }
    }

    pub fn pressed(&self) -> bool {
        self.state.is_pressed()
    }

    pub fn update(&mut self, event: &WindowEvent, size: Size, window_size: Size) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                // Make this baked into state.screen which can simplify the code below
                let (width_factor, height_factor) = factors(
                    size.width as f32,
                    size.height as f32,
                    window_size.width as f32,
                    window_size.height as f32,
                );

                // This really needs to be simplified
                self.pos.x = width_factor
                    * (position.x as f32 - (1.0 - width_factor) * window_size.width as f32 * 0.5);
                self.pos.y = height_factor
                    * (position.y as f32 + (1.0 - height_factor) * window_size.height as f32 * 0.5);
            }
            WindowEvent::CursorEntered { .. } => {
                self.inside = true;
            }
            WindowEvent::CursorLeft { .. } => {
                self.inside = false;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.state = *state;
                self.button = *button;
            }
            WindowEvent::MouseWheel { delta, phase, .. } => {
                self.delta = *delta;
                self.phase = *phase;
            }
            _ => {}
        }
    }
}
