#[derive(Debug)]
pub(crate) struct IDFactory {
    counter: u32,
}

impl IDFactory {
    pub const fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn next(&mut self) -> u32 {
        let id = self.counter;
        self.counter += 1;
        id
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }
}

// Move to renderable.rs?
pub(crate) static mut RENDERABLE_ID_FACTORY: IDFactory = IDFactory::new();
