use std::ops::Deref;
use std::fmt::Debug;

use std::cell::UnsafeCell;
use std::ptr::replace;
use std::rc::Rc;

// pub enum Thunk<'a, T: 'a + Debug + PartialEq + Clone> {
pub enum Thunk<T: Debug + PartialEq + Clone, F: FnOnce() -> T> {
    Suspend(F),
    Progress,
    Evaluated(T),
}

use self::Thunk::*;

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
                _ => (),
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

mod tests {
    use super::*;

    #[test]
    fn test_thunk() {
        let actual = Delay::new(|| {
                                    println!("Evaluate once.");
                                    10
                                });
        assert!(*actual == 10);
        assert!(*actual == 10);
    }
}

