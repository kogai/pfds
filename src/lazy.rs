use std::ops::Deref;
use std::fmt::{self, Debug, Formatter};
use std::cell::UnsafeCell;
use std::ptr::replace;
use std::cmp::PartialEq;
use std::clone::Clone;
use std::rc::Rc;


use self::Thunk::*;

pub enum Thunk<'a, T: Debug + PartialEq + Clone> {
    Suspend(Rc<'a + Fn() -> T>),
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

impl<'a, T: Debug + PartialEq + Clone> Clone for Thunk<'a, T> {
    fn clone(&self) -> Self {
        match *self {
            Suspend(ref suspention) => Suspend(suspention.clone()),
            Progress => Progress,
            Evaluated(ref v) => Evaluated(v.clone()),
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
    pub delay: UnsafeCell<Thunk<'a, T>>,
}

impl<'a, T: Debug + PartialEq + Clone> PartialEq for Susp<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.force();
        other.force();
        unsafe {
            match (&*self.delay.get(), &*other.delay.get()) {
                (&Evaluated(ref x), &Evaluated(ref y)) => x == y,
                _ => unreachable!(),
            }
        }
    }
}

impl<'a, T: Debug + PartialEq + Clone> Clone for Susp<'a, T> {
    fn clone(&self) -> Self {
        let thunk = unsafe { &*self.delay.get() };
        Susp { delay: UnsafeCell::new(thunk.clone()) }
    }
}

impl<'a, T: Debug + PartialEq + Clone> Susp<'a, T> {
    pub fn new<F: 'a + Fn() -> T>(f: F) -> Self {
        Susp { delay: UnsafeCell::new(Suspend(Rc::new(f))) }
    }

    pub fn thunk(&self) -> &Thunk<'a, T> {
        unsafe { &*self.delay.get() }
    }

    pub fn unwrap(&self) -> T {
        self.force();
        match self.thunk() {
            &Evaluated(ref v) => v.clone(),
            _ => unreachable!(),
        }
    }

    pub fn force(&self) {
        unsafe {
            match replace(self.delay.get(), Progress) {
                Suspend(susp) => {
                    *self.delay.get() = Evaluated(susp());
                },
                Progress => unreachable!(),
                evaluated => *self.delay.get() = evaluated,
            };
        }
    }
}

impl<'a, T: Debug + PartialEq + Clone> Deref for Susp<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.force();

        match unsafe { &*self.delay.get() } {
            &Evaluated(ref x) => &x,
            _ => unreachable!(),
        }
    }
}

mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    enum LazyMatch {
        One,
        Two
    }

    #[test]
    fn test_lazy_match() {
        use self::LazyMatch::*;

        let anonymous = susp!({
            println!("Evaluated only once.");
            One
        });

        let actual = match anonymous.thunk() {
            &Suspend(ref sus) => {
                println!("Expect not evaluated yet.");
                susp!(sus() != Two)
            },
            _ => unimplemented!(),
        };
        // Expect not evaluate match yet here.
        assert!(*actual); // Expect evaluate.
    }

    #[test]
    fn test_clone() {
        let actual = susp!(10).clone();
        assert!(actual != susp!(20));
        assert!(actual == susp!(10));
    }

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

