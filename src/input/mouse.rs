use glam::Vec2;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
};

#[derive(Debug, Copy, Clone)]
pub struct Mouse {
    pub inside: bool,
    pub pos: Option<Vec2>,
    pub state: ElementState,
    pub button: MouseButton,
    pub delta: MouseScrollDelta,
    pub phase: TouchPhase,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            inside: false,
            pos: None,
            state: ElementState::Released,
            button: MouseButton::Left,
            delta: MouseScrollDelta::PixelDelta(PhysicalPosition::new(0.0, 0.0)),
            phase: TouchPhase::Ended,
        }
    }

    pub fn pressed(&self) -> bool {
        self.state.is_pressed()
    }

    pub fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.pos = Some(Vec2::new(position.x as f32, position.y as f32));
            }
            WindowEvent::CursorEntered { .. } => {
                self.inside = true;
            }
            WindowEvent::CursorLeft { .. } => {
                self.pos = None;
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
