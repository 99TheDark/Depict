pub(crate) struct IDFactory {
    counter: u32,
}

impl IDFactory {
    pub fn new() -> Self {
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

pub(crate) static mut ID_FACTORY: IDFactory = IDFactory { counter: 0 };
