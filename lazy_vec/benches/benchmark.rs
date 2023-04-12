use criterion::{criterion_group, criterion_main, Criterion};
use lazy_vec::Vec3;

fn lazy_vec_add(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let b = Vec3::new(34.7, 2.6, 20.9);
    let _ = a + b;
}

fn normal_vec_add(){
   //TODO
}

fn lazy_vec_add_large(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let b = Vec3::new(34.7, 2.6, 20.9);
    let c = Vec3::new(78.6, 2.3, 495.0);
    let d = Vec3::new(23.4, 53.94, 75.1);
    let e = Vec3::new(0.002, 0.00005, 0.008);
    let f = Vec3::new(-23.6, 0.1, 1000.0);
    let g = Vec3::new(-1.4, -2.52, -40.3);
    let h = Vec3::new(-0.234, 123.536, 23.0);
    let _ = a + b + c + d + e + f + g + h;
}

fn normal_vec_add_large(){
    //TODO
}

fn lazy_vec_sub(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let b = Vec3::new(34.7, 2.6, 20.9);
    let _ = a - b;
}

fn normal_vec_sub(){
    //TODO
}

fn lazy_vec_sub_large(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let b = Vec3::new(34.7, 2.6, 20.9);
    let c = Vec3::new(78.6, 2.3, 495.0);
    let d = Vec3::new(23.4, 53.94, 75.1);
    let e = Vec3::new(0.002, 0.00005, 0.008);
    let f = Vec3::new(-23.6, 0.1, 1000.0);
    let g = Vec3::new(-1.4, -2.52, -40.3);
    let h = Vec3::new(-0.234, 123.536, 23.0);
    let _ = a - b - c - d - e - f - g - h;
}

fn normal_vec_sub_large(){
    //TODO
}

fn lazy_vec_scale(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let _ = 3.2 * a;
}

fn normal_vec_scale(){
    //TODO
}

fn lazy_vec_mul(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let b = Vec3::new(34.7, 2.6, 20.9);
    let _ = a * b;
}

fn normal_vec_mul(){
    //TODO
}

fn lazy_vec_mul_large(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let b = Vec3::new(34.7, 2.6, 20.9);
    let c = Vec3::new(78.6, 2.3, 495.0);
    let d = Vec3::new(23.4, 53.94, 75.1);
    let e = Vec3::new(0.002, 0.00005, 0.008);
    let f = Vec3::new(-23.6, 0.1, 1000.0);
    let g = Vec3::new(-1.4, -2.52, -40.3);
    let h = Vec3::new(-0.234, 123.536, 23.0);
    let _ = a * b * c * d * e * f * g * h;
}

fn normal_vec_mul_large(){
    //TODO
}

// Benchmark the vector functions in a criterion group
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lazy_vec");
    group.bench_function("lazy_vec_add", |b| b.iter(|| lazy_vec_add()));
    group.bench_function("normal_vec_add", |b| b.iter(|| normal_vec_add()));
    group.bench_function("lazy_vec_sub", |b| b.iter(|| lazy_vec_sub()));
    group.bench_function("normal_vec_sub", |b| b.iter(|| normal_vec_sub()));
    group.bench_function("lazy_vec_scale", |b| b.iter(|| lazy_vec_scale()));
    group.bench_function("normal_vec_scale", |b| b.iter(|| normal_vec_scale()));
    group.bench_function("lazy_vec_mul", |b| b.iter(|| lazy_vec_mul()));
    group.bench_function("normal_vec_mul", |b| b.iter(|| normal_vec_mul()));
    group.bench_function("lazy_vec_add_large", |b| b.iter(|| lazy_vec_add_large()));
    group.bench_function("normal_vec_add_large", |b| b.iter(|| normal_vec_add_large()));
    group.bench_function("lazy_vec_sub_large", |b| b.iter(|| lazy_vec_sub_large()));
    group.bench_function("normal_vec_sub_large", |b| b.iter(|| normal_vec_sub_large()));
    group.bench_function("lazy_vec_mul_large", |b| b.iter(|| lazy_vec_mul_large()));
    group.bench_function("normal_vec_mul_large", |b| b.iter(|| normal_vec_mul_large()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);