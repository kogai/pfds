use std::ops::Deref;
use std::fmt::{self, Debug, Formatter};
use std::cell::UnsafeCell;
use std::ptr::replace;

pub enum Thunk<T: Debug + PartialEq + Clone, F: FnOnce() -> T> {
    Suspend(F),
    Progress,
    Evaluated(T),
}

impl<T: Debug + PartialEq + Clone, F: FnOnce() -> T> Debug for Thunk<T, F> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Suspend(_) => write!(f, "Suspend {{ (not yet...) }}"),
            &Progress => write!(f, "Progress"),
            &Evaluated(ref v) => write!(f, "Evaluated {{ {:?} }}", v),
        }
    }
}

use self::Thunk::*;

#[derive(Debug)]
pub struct Delay<T: Debug + PartialEq + Clone, F: FnOnce() -> T> {
    thunk: UnsafeCell<Thunk<T, F>>,
}

impl<T: Debug + PartialEq + Clone, F: FnOnce() -> T> Delay<T, F> {
    fn new(f: F) -> Self {
        Delay { thunk: UnsafeCell::new(Suspend(f)) }
    }

    fn force(&self) {
        unsafe {
            match replace(self.thunk.get(), Progress) {
                Suspend(susp) => *self.thunk.get() = Evaluated(susp()),
                Progress => unreachable!(),
                evaluated => *self.thunk.get() = evaluated,
            };
        }
    }
}

impl<T: Debug + PartialEq + Clone, F: FnOnce() -> T> Deref for Delay<T, F> {
    type Target = T;

    fn deref(&self) -> &T {
        self.force();

        match unsafe { &*self.thunk.get() } {
            &Evaluated(ref x) => &x,
            _ => unreachable!(),
        }
    }
}

#[macro_export]
macro_rules! lazy {
    ($e:expr) => {
        self::Delay::new(move || { $e })
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_thunk_macro() {
        let actual = lazy!({
                               println!("Evaluate only once!");
                               10
                           });
        assert!(*actual == 10);
        assert!(*actual == 10);
        assert!(*actual == 10);
    }

    #[test]
    fn test_thunk() {
        let actual = Delay::new(move || {
                                    println!("Evaluate only once!");
                                    10
                                });
        assert!(*actual == 10);
        assert!(*actual == 10);
        assert!(*actual == 10);
    }
}

