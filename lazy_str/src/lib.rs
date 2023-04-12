use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

pub struct LazyStr<T: Display>(T);

pub struct StrNode<A: Display, B: Display> {
    a: A,
    b: B,
}

// Constructor
impl<T: Display> LazyStr<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

// When implementing the Display trait, the .to_string() method can be called on the struct
impl<T: Display> Display for LazyStr<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        // For our container node, we just call fmt() on the generic type
        self.0.fmt(f)?;
        Ok(())
    }
}

// We must also implement the Display trait for the inner nodes
impl<A: Display, B: Display> Display for StrNode<A, B> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        // We can just call fmt() on each struct member. Since we use the same formatter, both
        // strings are written after another
        self.a.fmt(f)?;
        self.b.fmt(f)?;
        Ok(())
    }
}

// Implementing the add trait let's us use the '+' operator
impl<S: Display, T: Display> Add<S> for LazyStr<T> {
    type Output = LazyStr<StrNode<T, S>>;
    #[inline]
    fn add(self, other: S) -> Self::Output {
        LazyStr(StrNode {
            a: self.0,
            b: other,
        })
    }
}

// #[test]
// fn string_test() {
//     let a = LazyStr::new("foo", "bar");
//     assert_eq!("foobar", a.to_string());
// }
