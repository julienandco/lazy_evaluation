use std::ops::{Add, Mul, Sub};

// Vector Algebra
pub trait Vec3Expr {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

// This is out container struct which hold the abstract syntax tree
pub struct LazyVec3<T>(T);

// The following are syntax tree nodes. They merley hold the data to be evaluated as soon as
// Vec3::from is called
pub struct Vec3Add<L, R>(L, R);
pub struct Vec3Sub<L, R>(L, R);
pub struct Vec3Scal<L, R>(L, R);
pub struct Vec3Mul<L, R>(L, R);

// We create an alias for a lazy evaluated vector with f64 floats
pub type Vec3 = LazyVec3<(f64, f64, f64)>;

// Constructor for Vec3
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self((x, y, z))
    }
}

// The Vec3Expr trait promises an interface to get all tree coordinates of the vector. For a simple
// Vec3, we just return the value of the tuple. Easy
impl Vec3Expr for Vec3 {
    #[inline]
    fn x(&self) -> f64 {
        (self.0).0
    }
    #[inline]
    fn y(&self) -> f64 {
        (self.0).1
    }
    #[inline]
    fn z(&self) -> f64 {
        (self.0).2
    }
}

// The From trait evaluates the abstract syntax tree of a LazyVec3 container struct and returns a
// Vec3. Hence it resolves the lazy evaluation. Since every node of the abstract syntax tree
// implements the Vec3Expr trait, we can call the x(), y() and z() functions which in turn
// recursively resolve their values. When using the #[inline] macro, no function calls are made.
impl<T: Vec3Expr> From<LazyVec3<T>> for Vec3 {
    fn from(s: LazyVec3<T>) -> Self {
        Self::new(s.0.x(), s.0.y(), s.0.z())
    }
}

// Addition
// This impl block defines how to resolve an Vec3Add block
impl<L: Vec3Expr, R: Vec3Expr> Vec3Expr for Vec3Add<L, R> {
    #[inline]
    fn x(&self) -> f64 {
        self.0.x() + self.1.x()
    }
    #[inline]
    fn y(&self) -> f64 {
        self.0.y() + self.1.y()
    }
    #[inline]
    fn z(&self) -> f64 {
        self.0.z() + self.1.z()
    }
}

// Operator overloading of the '+' sign for Vec3 + Vec3Expr
impl<R: Vec3Expr> Add<R> for Vec3 {
    type Output = LazyVec3<Vec3Add<Self, R>>;
    #[inline]
    fn add(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Add(self, rhs))
    }
}

// Operator overloading of the '+' sign for Vec3Expr + Vec3Expr
impl<L: Vec3Expr, R: Vec3Expr> Add<R> for LazyVec3<L> {
    type Output = LazyVec3<Vec3Add<L, R>>;
    #[inline]
    fn add(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Add(self.0, rhs))
    }
}

#[test]
fn add_test_1() {
    let a = Vec3::new(0.0, 0.0, 1.0);
    let b = Vec3::new(0.0, 1.0, 0.0);
    let c = Vec3::new(1.0, 0.0, 0.0);
    let d = Vec3::new(0.0, 0.0, 1.0);
    let e = Vec3::new(0.0, 1.0, 0.0);
    let f = Vec3::new(1.0, 0.0, 0.0);
    let r = a + b + c + d + e + f;
    let r = Vec3::from(r);
    assert_eq!((2.0, 2.0, 2.0), (r.x(), r.y(), r.z()));
}

// Subtraction
impl<L: Vec3Expr, R: Vec3Expr> Vec3Expr for Vec3Sub<L, R> {
    #[inline]
    fn x(&self) -> f64 {
        self.0.x() - self.1.x()
    }
    #[inline]
    fn y(&self) -> f64 {
        self.0.y() - self.1.y()
    }
    #[inline]
    fn z(&self) -> f64 {
        self.0.z() - self.1.z()
    }
}

impl<R: Vec3Expr> Sub<R> for Vec3 {
    type Output = LazyVec3<Vec3Sub<Self, R>>;
    #[inline]
    fn sub(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Sub(self, rhs))
    }
}

impl<L: Vec3Expr, R: Vec3Expr> Sub<R> for LazyVec3<L> {
    type Output = LazyVec3<Vec3Sub<L, R>>;
    #[inline]
    fn sub(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Sub(self.0, rhs))
    }
}

// Scalar Multiplication

// Rhs scalar
impl<L: Vec3Expr> Vec3Expr for Vec3Scal<L, f64> {
    #[inline]
    fn x(&self) -> f64 {
        self.1 * self.0.x()
    }
    #[inline]
    fn y(&self) -> f64 {
        self.1 * self.0.y()
    }
    #[inline]
    fn z(&self) -> f64 {
        self.1 * self.0.z()
    }
}

impl Mul<f64> for Vec3 {
    type Output = LazyVec3<Vec3Scal<Self, f64>>;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        LazyVec3(Vec3Scal(self, rhs))
    }
}

impl<L: Vec3Expr> Mul<f64> for LazyVec3<L> {
    type Output = LazyVec3<Vec3Scal<L, f64>>;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        LazyVec3(Vec3Scal(self.0, rhs))
    }
}

// Lhs scalar
impl<R: Vec3Expr> Vec3Expr for Vec3Scal<f64, R> {
    #[inline]
    fn x(&self) -> f64 {
        self.0 * self.1.x()
    }
    #[inline]
    fn y(&self) -> f64 {
        self.0 * self.1.y()
    }
    #[inline]
    fn z(&self) -> f64 {
        self.0 * self.1.z()
    }
}

impl Mul<Vec3> for f64 {
    type Output = LazyVec3<Vec3Scal<Self, Vec3>>;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        LazyVec3(Vec3Scal(self, rhs))
    }
}

impl<R: Vec3Expr> Mul<LazyVec3<R>> for f64 {
    type Output = LazyVec3<Vec3Scal<f64, R>>;
    #[inline]
    fn mul(self, rhs: LazyVec3<R>) -> Self::Output {
        LazyVec3(Vec3Scal(self, rhs.0))
    }
}

// Vector Product
impl<L: Vec3Expr, R: Vec3Expr> Vec3Expr for Vec3Mul<L, R> {
    #[inline]
    fn x(&self) -> f64 {
        self.0.y() * self.1.z() - self.0.z() * self.1.y()
    }
    #[inline]
    fn y(&self) -> f64 {
        self.0.z() * self.1.x() - self.0.x() * self.1.z()
    }
    #[inline]
    fn z(&self) -> f64 {
        self.0.x() * self.1.y() - self.0.y() * self.1.x()
    }
}

impl<R: Vec3Expr> Mul<R> for Vec3 {
    type Output = LazyVec3<Vec3Mul<Self, R>>;
    #[inline]
    fn mul(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Mul(self, rhs))
    }
}

impl<L: Vec3Expr, R: Vec3Expr> Mul<R> for LazyVec3<L> {
    type Output = LazyVec3<Vec3Mul<L, R>>;
    #[inline]
    fn mul(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Mul(self.0, rhs))
    }
}