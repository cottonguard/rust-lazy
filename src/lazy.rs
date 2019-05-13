pub enum Lazy<F: Fn() -> T, T> {
    Thunk(F),
    Value(T)
}

use Lazy::*;

impl<T> Lazy<fn() -> T, T> {
    pub fn value(v: T) -> Lazy<fn() -> T, T> {
        Value(v)
    }
}

impl<F: Fn() -> T, T> Lazy<F, T> {
    pub fn thunk(f: F) -> Lazy<F, T> {
        Thunk(f)
    }

    fn force(&mut self) {
        if let Thunk(f) = self {
            *self = Value(f());
        }
    }

    pub fn get(&mut self) -> &T {
        self.force();
        match self {
            Value(v) => v,
            Thunk(_) => unreachable!()
        }
    }

    pub fn get_if_forced(&self) -> Option<&T> {
        match self {
            Value(v) => Some(v),
            Thunk(_) => None
        }
    }

    pub fn is_forced(&self) -> bool {
        match self {
            Value(_) => true,
            Thunk(_) => false
        }
    }

    pub fn into_inner(mut self) -> T {
        self.force();
        match self {
            Value(v) => v,
            Thunk(_) => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = 123;
        let mut t = Thunk(|| 2 * x);
        assert!(t.is_forced() == false);
        assert!(t.get_if_forced() == None);
        assert!(t.get() == &246);
        assert!(t.is_forced() == true);
        assert!(t.get_if_forced() == Some(&246));
        assert!(t.get() == &246);
        assert!(t.into_inner() == 246);

        let mut v = Lazy::value(234);
    }
}