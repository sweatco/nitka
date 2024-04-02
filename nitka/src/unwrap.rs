use std::ops::{Deref, DerefMut};

/// Auto unwrapping optional.
/// Use only in test context!
#[derive(Debug)]
pub struct Unwrap<T> {
    val: Option<T>,
}

impl<T> Default for Unwrap<T>  {
    fn default() -> Self {
        Self { val: None }
    }
}

impl<T> From<T> for Unwrap<T> {
    fn from(value: T) -> Self {
        Self { val: value.into() }
    }
}

impl<T> Deref for Unwrap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.val.as_ref().unwrap()
    }
}

impl<T> DerefMut for Unwrap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.val.as_mut().unwrap()
    }
}
