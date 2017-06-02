use std::ops::Deref;
use std::fmt::{self, Debug, Formatter};
use std::cell::UnsafeCell;
use std::ptr::replace;
use std::cmp::PartialEq;

use self::Thunk::*;
pub enum Thunk<'a, T: Debug + PartialEq + Clone> {
    Suspend(Box<'a + Fn() -> T>),
    Progress,
    Evaluated(T),
}

impl<'a, T: Debug + PartialEq + Clone> Debug for Thunk<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Suspend(_) => write!(f, "Suspend {{ (not yet...) }}"),
            &Progress => write!(f, "Progress"),
            &Evaluated(ref v) => write!(f, "Evaluated {{ {:?} }}", v),
        }
    }
}


#[macro_export]
macro_rules! susp {
    ($e:expr) => {
        self::Susp::new(move || { $e })
    }
}

#[derive(Debug)]
pub struct Susp<'a, T: Debug + PartialEq + Clone> {
    thunk: UnsafeCell<Thunk<'a, T>>,
}

impl<'a, T: Debug + PartialEq + Clone> PartialEq for Susp<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.force();
        other.force();
        unsafe {
            match (&*self.thunk.get(), &*other.thunk.get()) {
                (&Evaluated(ref x), &Evaluated(ref y)) => x == y,
                _ => unreachable!(),
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<'a, T: Debug + PartialEq + Clone> Susp<'a, T> {
    pub fn new<F: 'a + Fn() -> T>(f: F) -> Self {
        Susp { thunk: UnsafeCell::new(Suspend(box f)) }
    }

    pub fn force(&self) {
        unsafe {
            match replace(self.thunk.get(), Progress) {
                Suspend(susp) => {
                    *self.thunk.get() = Evaluated(susp());
                },
                Progress => unreachable!(),
                evaluated => *self.thunk.get() = evaluated,
            };
        }
    }
}

impl<'a, T: Debug + PartialEq + Clone> Deref for Susp<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.force();

        match unsafe { &*self.thunk.get() } {
            &Evaluated(ref x) => &x,
            _ => unreachable!(),
        }
    }
}

// TODO: add fmt to show inner state of Thunk

mod tests {
    use super::*;

    #[test]
    fn test_ne() {
        assert!(susp!(10) != susp!(20));
        assert!(!(susp!(10) != susp!(10)));
    }

    #[test]
    fn test_eq() {
        assert!(susp!(10) == susp!(10));
        assert!(!(susp!(10) == susp!(20)));
    }

    fn plus<'a>(x: Susp<'a, i32>, y: Susp<'a, i32>) -> Susp<'a, i32> {
        susp!({
            println!("Evaluate only once!");
            *x + *y
        })
    }

    #[test]
    fn test_lazy() {
        let actual = plus(susp!(10), susp!(20));
        println!("Before evaluate");

        assert!(*actual == 30);
        assert!(*actual == 30);
        assert!(*actual == 30);
    }

    #[test]
    fn test_susp() {
        let actual = susp!({
                               println!("Evaluate only once!");
                                10
                           });
        assert!(*actual == 10);
        assert!(*actual == 10);
        assert!(*actual == 10);
    }

    #[test]
    fn test_thunk() {
        let actual = Susp::new(move || {
                                   println!("Evaluate only once!");
                                   10
                               });
        assert!(*actual == 10);
        assert!(*actual == 10);
        assert!(*actual == 10);
    }
}

