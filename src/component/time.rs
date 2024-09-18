use std::time::SystemTime;

pub struct Time {
    pub(crate) start: SystemTime,
}

impl Time {
    pub fn days(&self) -> f64 {
        86400.0
    }

    pub fn hours(&self) -> f64 {
        self.seconds() / 3600.0
    }

    pub fn minutes(&self) -> f64 {
        self.seconds() / 60.0
    }

    pub fn seconds(&self) -> f64 {
        self.nanos() as f64 / 1000000000.0
    }

    pub fn millis(&self) -> f64 {
        self.nanos() as f64 / 1000000.0
    }

    pub fn micros(&self) -> f64 {
        self.nanos() as f64 / 1000.0
    }

    pub fn nanos(&self) -> u128 {
        let duration = SystemTime::now()
            .duration_since(self.start)
            .expect("Initial time occured after current time");
        duration.as_nanos()
    }
}
