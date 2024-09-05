pub struct Tracker<T: Clone> {
    pub last: T,
    cur: T,
}

impl<T: Clone> Tracker<T> {
    pub fn new(tracked: T) -> Tracker<T> {
        Tracker {
            last: tracked.clone(),
            cur: tracked,
        }
    }

    pub fn step(&mut self) {
        self.last = self.cur.clone();
    }
}

impl<T: Clone> std::ops::Deref for Tracker<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.cur
    }
}

impl<T: Clone> std::ops::DerefMut for Tracker<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cur
    }
}
