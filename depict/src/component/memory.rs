use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct Memory<T: Debug> {
    pub(crate) value: T,
    pub(crate) remembered: bool,
}

impl<T: Debug> Memory<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            remembered: false,
        }
    }
}

impl<T: Debug> std::ops::Deref for Memory<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Debug> std::ops::DerefMut for Memory<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
