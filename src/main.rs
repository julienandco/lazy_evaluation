use std::ops::{Add, Mul, Sub};

// Vector Algebra
trait Vec3Expr {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

struct LazyVec3<T>(T);
struct Vec3Add<L,R>(L,R);
struct Vec3Subtract<L,R>(L,R);
// TODO: Scalar Multiplication (lhs and rhs)
// struct Vec3Scale<f64,R>(f64,R);
struct Vec3Mul<L,R>(L,R);
type Vec3 = LazyVec3<(f64,f64,f64)>;

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self { Self((x, y, z)) }
}

impl Vec3Expr for Vec3 {
    fn x(&self) -> f64 { (self.0).0 }
    fn y(&self) -> f64 { (self.0).1 }
    fn z(&self) -> f64 { (self.0).2 }
}

impl<T: Vec3Expr> From<LazyVec3<T>> for Vec3 {
    fn from(s: LazyVec3<T>) -> Self {
        Self::new(s.0.x(), s.0.y(), s.0.z())
    }
}

// Addition
impl<L: Vec3Expr, R: Vec3Expr> Vec3Expr for Vec3Add<L,R> {
    fn x(&self) -> f64 { self.0.x() + self.1.x() }
    fn y(&self) -> f64 { self.0.y() + self.1.y() }
    fn z(&self) -> f64 { self.0.z() + self.1.z() }
}

impl<R: Vec3Expr> Add<R> for Vec3 {
    type Output = LazyVec3<Vec3Add<Self,R>>;
    fn add(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Add(self, rhs))
    }
}

impl<L: Vec3Expr, R: Vec3Expr> Add<R> for LazyVec3<L> {
    type Output = LazyVec3<Vec3Add<L,R>>;
    fn add(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Add(self.0, rhs))
    }
}

// Subtraction
impl<L: Vec3Expr, R: Vec3Expr> Vec3Expr for Vec3Subtract<L,R> {
    fn x(&self) -> f64 { self.0.x() - self.1.x() }
    fn y(&self) -> f64 { self.0.y() - self.1.y() }
    fn z(&self) -> f64 { self.0.z() - self.1.z() }
}

impl<R: Vec3Expr> Sub<R> for Vec3 {
    type Output = LazyVec3<Vec3Subtract<Self,R>>;
    fn sub(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Subtract(self, rhs))
    }
}

impl<L: Vec3Expr, R: Vec3Expr> Sub<R> for LazyVec3<L> {
    type Output = LazyVec3<Vec3Subtract<L,R>>;
    fn sub(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Subtract(self.0, rhs))
    }
}

// TODO: Scalar Multiplication

// The following was intended to solve lhs scalar multiplication: alpha * v, but it did not work 🥲
/*
impl<L: f64, R: Vec3Expr> Vec3Expr for Vec3Scale<L,R> {
    fn x(&self) -> f64 { (self.0).0 * self.1.x() }
    fn y(&self) -> f64 { (self.0).0 - self.1.y() }
    fn z(&self) -> f64 { (self.0).0 - self.1.z() }
}

impl<L: f64> Mul<L> for Vec3 {
    type Output = LazyVec3<Vec3Scale<L, Self>>;
    fn mul(self, lhs: L) -> Self::Output {
        LazyVec3(Vec3Scale(lhs, self))
    }
}

impl<L: f64, R: Vec3Expr> Mul<L> for LazyVec3<R> {
    type Output = LazyVec3<Vec3Scale<L,R>>;
    fn mul(self, lhs: L) -> Self::Output {
        LazyVec3(Vec3Scale(lhs, self.1))
    }
}*/

// Vector Product
impl<L: Vec3Expr, R: Vec3Expr> Vec3Expr for Vec3Mul<L,R> {
    fn x(&self) -> f64 { self.0.y() * self.1.z() - self.0.z() * self.1.y() }
    fn y(&self) -> f64 { self.0.z() * self.1.x() - self.0.x() * self.1.z() }
    fn z(&self) -> f64 { self.0.x() * self.1.y() - self.0.y() * self.1.x() }
}

impl<R: Vec3Expr> Mul<R> for Vec3 {
    type Output = LazyVec3<Vec3Mul<Self,R>>;
    fn mul(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Mul(self, rhs))
    }
}

impl<L: Vec3Expr, R: Vec3Expr> Mul<R> for LazyVec3<L> {
    type Output = LazyVec3<Vec3Mul<L,R>>;
    fn mul(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Mul(self.0, rhs))
    }
}

// TODO: String library

fn main() {
    let va = Vec3::new(0.0, 0.0, 1.0);
    let vb = Vec3::new(0.0, 1.0, 0.0);
    let vc = Vec3::new(1.0, 0.0, 0.0);
    //let vx = va + vb + vc;
    //let vy = va - vb - vc;
    //let vz = va * vb;
    let vzz = va * vb;
    let z = Vec3::from(vy);
    println!("z = ({}, {}, {})", z.x(), z.y(), z.z());
}
