use std::ops::Add;

trait Vec3Expr {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

struct LazyVec3<T>(T);
struct Vec3Add<L,R>(L,R);
type Vec3 = LazyVec3<(f64,f64,f64)>;

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self { Self((x, y, z)) }
}

impl Vec3Expr for Vec3 {
    fn x(&self) -> f64 { (self.0).0 }
    fn y(&self) -> f64 { (self.0).1 }
    fn z(&self) -> f64 { (self.0).2 }
}

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

impl<T: Vec3Expr> From<LazyVec3<T>> for Vec3 {
    fn from(s: LazyVec3<T>) -> Self {
        Self::new(s.0.x(), s.0.y(), s.0.z())
    }
}

impl<L: Vec3Expr, R: Vec3Expr> Add<R> for LazyVec3<L> {
    type Output = LazyVec3<Vec3Add<L,R>>;
    fn add(self, rhs: R) -> Self::Output {
        LazyVec3(Vec3Add(self.0, rhs))
    }
}

fn main() {
    let va = Vec3::new(0.0, 0.0, 1.0);
    let vb = Vec3::new(0.0, 1.0, 0.0);
    let vc = Vec3::new(1.0, 0.0, 0.0);
    let vx = va + vb + vc;
}