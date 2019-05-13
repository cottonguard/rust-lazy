use std::cell::UnsafeCell;
use std::ops::Deref;

use crate::lazy::*;

struct LazyCell<F: Fn() -> T, T> {
    inner: UnsafeCell<Lazy<F, T>>
}

impl<F: Fn() -> T, T> LazyCell<F, T> {
    fn thunk(f: F) -> LazyCell<F, T> {
        LazyCell {
            inner: UnsafeCell::new(Lazy::thunk(f))
        }
    }

    fn get(&self) -> &T {
        unsafe {
            (&mut *self.inner.get()).get()
        }
    }
    
    fn into_inner(self) -> T {
        self.inner.into_inner().into_inner()
    }
}

impl<T> LazyCell<fn() -> T, T> {
    fn value(v: T) -> LazyCell<fn() -> T, T> {
        LazyCell {
            inner: UnsafeCell::new(Lazy::value(v))
        }
    }
}

impl<F: Fn() -> T, T> AsRef<T> for LazyCell<F, T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<F: Fn() -> T, T> Deref for LazyCell<F, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = 123;
        let lc = LazyCell::thunk(|| 2 * x);
        assert!(lc.get() == &246);
        assert!(*lc == 246);
        assert!(lc.into_inner() == 246);
    }
}