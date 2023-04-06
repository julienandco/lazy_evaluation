use crate::vector_algebra::Vec3;
use crate::vector_algebra::Vec3Expr;

mod vector_algebra;
mod string_library;

fn main() {
    let va = Vec3::new(0.0, 0.0, 1.0);
    let vb = Vec3::new(0.0, 1.0, 0.0);
    let vc = Vec3::new(1.0, 0.0, 0.0);
    //let vx = va + vb + vc;
    //let vy = va - vb - vc;
    let vz = va * vb;
    let vzz = vc * 3.5;
    let z = Vec3::from(vz);
    let zz = Vec3::from(vzz);
    println!("z = ({}, {}, {})", z.x(), z.y(), z.z());
    println!("zz = ({}, {}, {})", zz.x(), zz.y(), zz.z());
}
