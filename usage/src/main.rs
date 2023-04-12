use lazy_str::LazyStr;
// use lazy_vec::Vec3;

fn main() {
    // let va = Vec3::new(0.0, 0.0, 1.0);
    // let vb = Vec3::new(0.0, 1.0, 0.0);
    // let vc = Vec3::new(1.0, 0.0, 0.0);
    // //let vx = va + vb + vc;
    // //let vy = va - vb - vc;
    // let vz = va * vb;
    // // let vzz = vc * 3.5;
    // let vzz = 3.5 * vc * 2.0;
    // let z = Vec3::from(vz);
    // let zz = Vec3::from(vzz);
    // println!("z = ({}, {}, {})", z.x(), z.y(), z.z());
    // println!("zz = ({}, {}, {})", zz.x(), zz.y(), zz.z());



    let a = LazyStr::new("foo");
    // let b = LazyStr::new("bazz", "bizz");
    let r = a + "bazz" + "bizz";
    // let rr = r.append("buzz");

    println!("{}", r.to_string());
}
