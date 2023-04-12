use criterion::{criterion_group, criterion_main, Criterion};
use lazy_vec::Vec3;

use std::ops::{Add, Mul, Sub};

struct BadVec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for BadVec3 {
    type Output = BadVec3;
    fn add(self, other: Self) -> Self::Output {
        BadVec3{ x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for BadVec3 {
    type Output = BadVec3;
    fn sub(self, other: Self) -> Self::Output {
        BadVec3{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul<BadVec3> for BadVec3 {
    type Output = BadVec3;
    fn mul(self, other: Self) -> Self::Output {
        BadVec3{ x: self.y * other.z - self.z * other.y, y: self.z * other.x - self.x * other.z, z: self.x * other.y - self.y * other.x }
    }
}

impl Mul<f64> for BadVec3 {
    type Output = BadVec3;
    fn mul(self, other: f64) -> Self::Output {
        BadVec3{ x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl Mul<BadVec3> for f64 {
    type Output = BadVec3;
    fn mul(self, other: BadVec3) -> Self::Output {
        BadVec3{ x: self * other.x, y: self * other.y, z: self * other.z }
    }
}

fn lazy_vec_add(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let b = Vec3::new(34.7, 2.6, 20.9);
    let _ = a + b;
}

fn normal_vec_add(){
    let a = BadVec3{ x: 1.4, y: 2.5, z: 40.2 };
    let b = BadVec3{ x: 34.7, y: 2.6, z: 20.9 };
    let _ = a + b;
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
    let a = BadVec3{ x: 1.4, y: 2.5, z: 40.2 };
    let b = BadVec3{ x: 34.7, y: 2.6, z: 20.9 };
    let c = BadVec3{ x: 78.6, y: 2.3, z: 495.0 };
    let d = BadVec3{ x: 23.4, y: 53.94, z: 75.1 };
    let e = BadVec3{ x: 0.002, y: 0.00005, z: 0.008 };
    let f = BadVec3{ x: -23.6, y: 0.1, z: 1000.0 };
    let g = BadVec3{ x: -1.4, y: -2.52, z: -40.3 };
    let h = BadVec3{ x: -0.234, y: 123.536, z: 23.0 };
    let _ = a + b + c + d + e + f + g + h;
}

fn lazy_vec_sub(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let b = Vec3::new(34.7, 2.6, 20.9);
    let _ = a - b;
}

fn normal_vec_sub(){
    let a = BadVec3{ x: 1.4, y: 2.5, z: 40.2 };
    let b = BadVec3{ x: 34.7, y: 2.6, z: 20.9 };
    let _ = a - b;
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
    let a = BadVec3{ x: 1.4, y: 2.5, z: 40.2 };
    let b = BadVec3{ x: 34.7, y: 2.6, z: 20.9 };
    let c = BadVec3{ x: 78.6, y: 2.3, z: 495.0 };
    let d = BadVec3{ x: 23.4, y: 53.94, z: 75.1 };
    let e = BadVec3{ x: 0.002, y: 0.00005, z: 0.008 };
    let f = BadVec3{ x: -23.6, y: 0.1, z: 1000.0 };
    let g = BadVec3{ x: -1.4, y: -2.52, z: -40.3 };
    let h = BadVec3{ x: -0.234, y: 123.536, z: 23.0 };
    let _ = a - b - c - d - e - f - g - h;
}

fn lazy_vec_scale(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let _ = 3.2 * a;
}

fn normal_vec_scale(){
    let a = BadVec3{ x: 1.4, y: 2.5, z: 40.2 };
    let _ = 3.2 * a;
}

fn lazy_vec_mul(){
    let a = Vec3::new(1.4, 2.5, 40.2);
    let b = Vec3::new(34.7, 2.6, 20.9);
    let _ = a * b;
}

fn normal_vec_mul(){
    let a = BadVec3{ x: 1.4, y: 2.5, z: 40.2 };
    let b = BadVec3{ x: 34.7, y: 2.6, z: 20.9 };
    let _ = a * b;
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
    let a = BadVec3{ x: 1.4, y: 2.5, z: 40.2 };
    let b = BadVec3{ x: 34.7, y: 2.6, z: 20.9 };
    let c = BadVec3{ x: 78.6, y: 2.3, z: 495.0 };
    let d = BadVec3{ x: 23.4, y: 53.94, z: 75.1 };
    let e = BadVec3{ x: 0.002, y: 0.00005, z: 0.008 };
    let f = BadVec3{ x: -23.6, y: 0.1, z: 1000.0 };
    let g = BadVec3{ x: -1.4, y: -2.52, z: -40.3 };
    let h = BadVec3{ x: -0.234, y: 123.536, z: 23.0 };
    let _ = a * b * c * d * e * f * g * h;
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