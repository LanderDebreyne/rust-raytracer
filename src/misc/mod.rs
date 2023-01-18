use nalgebra::Vector3;
use rand::Rng;

pub fn random_unit_vector() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let a: f64 = rng.gen();
    let b: f64 = rng.gen();
    let z = 1.0 - 2.0 * a;
    let r = (1.0 - z * z).sqrt();
    let phi = 2.0 * std::f64::consts::PI * b;
    let x = r * phi.cos();
    let y = r * phi.sin();
    Vector3::new(x, y, z)
}

pub fn near_zero(v: &Vector3<f64>) -> bool {
    let s = 1e-8;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    *v - 2.0 * v.dot(n) * *n
}