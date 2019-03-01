use rand;

use crate::{ vec3, Vector3 };

pub const PI: f64 = std::f64::consts::PI;
pub const HALF_PI: f64 = 0.5 * PI;
pub const TWO_PI: f64 = 2.0 * PI;
pub const INV_PI: f64 = 1.0 / PI;
pub const GAMMA: f64 = 2.2;
pub const INV_GAMMA: f64 = 1.0 / GAMMA;

pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(v: Vector3, n: Vector3, ri: f64) -> Option<Vector3> {
    let dt = v.dot(n);
    let d = 1.0 - ri * ri * (1.0 - dt * dt);
    if d > 0.0 {
        Some(-ri * (v - n * dt) - n * d.sqrt())
    } else {
        None
    }
}

pub fn pow2(v: f64) -> f64 {
    v * v
}

pub fn pow3(v: f64) -> f64 {
    v * v * v
}

pub fn pow4(v: f64) -> f64 {
    let v2 = v * v;
    v2 * v2
}

pub fn pow5(v: f64) -> f64 {
    let v2 = v * v;
    v2 * v2 * v
}

pub fn schlick_fresnel(cosine: f64, ri: f64) -> f64 {
    let r0 = pow2((1.0 - ri) / (1.0 + ri));
    r0 + (1.0 - r0) * pow5(1.0 - cosine)
}

pub fn clamp(v: f64, min: f64, max: f64) -> f64 {
    v.min(max).max(min)
}

pub fn linear_to_gamma(v: f64) -> f64 {
    v.powf(INV_GAMMA)
}

pub fn gamma_to_linear(v: f64) -> f64 {
    v.powf(GAMMA)
}

pub fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let d = b * b - 4.0 * a * c;
    if d > 0.0 {
        let d_sqrt = d.sqrt();
        let t1 = (-b - d_sqrt) / (2.0 * a);
        let t2 = (-b + d_sqrt) / (2.0 * a);
        Some((t1, t2))
    } else {
        None
    }
}

pub fn sample_random_cosine_dir() -> (Vector3, f64) {
    let r1 = rand::random::<f64>();
    let r2 = rand::random::<f64>();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let dir = vec3(x, y, z);
    let pdf = z / PI;
    (dir, pdf)
}

pub fn change_basis(v: Vector3, n: Vector3) -> Vector3{
    let n = n.norm();
    let up = if n.x.abs() > 0.9 {
        vec3(0.0, 1.0, 0.0)
    } else {
        vec3(1.0, 0.0, 0.0)
    };
    let s = n.cross(up).norm();
    let t = s.cross(n);

    v.x * s + v.y * t + v.z * n
}

pub fn sphere_uv(v: Vector3) -> (f64, f64) {
    let phi = v.z.atan2(v.x);
    let theta = v.y.asin();
    (1.0 - (phi + PI) / TWO_PI, (theta + HALF_PI) / PI)
}