#![allow(dead_code)]

use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

pub struct LazyStr<A, B>
where
    A: Display,
    B: Display,
{
    a: A,
    b: B,
}

// Constructor
impl<A, B> LazyStr<A, B> 
where
    A: Display,
    B: Display,
{
    pub fn new(a: A, b: B) -> LazyStr<A, B> {
        LazyStr { a, b }
    }
}

// When implementing the Display trait, the .to_string method can be called on the struct
impl<A, B> Display for LazyStr<A, B> 
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        // We can just call fmt() on each struct member. Since we use the same formatter, both
        // strings are written after another
        self.a.fmt(f)?;
        self.b.fmt(f)?;
        Ok(())
    }
}

#[test]
fn string_test() {
    let a = LazyStr::new("foo", "bar");
    assert_eq!("foobar", a.to_string());
}

// Implementing the add trait let's us use the '+' operator
impl<A, B, C> Add<C> for LazyStr<A, B>
where
    A: Display,
    B: Display,
    C: Display,
{
    type Output = LazyStr<LazyStr<A, B>, C>;
    fn add(self, other: C) -> Self::Output {
        LazyStr::new(self, other)
    }
}
