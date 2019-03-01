use std::ops::{ Add, Sub, Mul, Div, Neg };
use std::ops::{ AddAssign, SubAssign, MulAssign, DivAssign };

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }
    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
    pub fn mag(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn sq_mag(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn norm(self) -> Vector3 {
        self / self.mag()
    }
    pub fn dot(self, v: Vector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    pub fn cross(self, v:Vector3) -> Vector3 {
        Vector3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x
        )
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, v: Vector3) -> Vector3 {
        Vector3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, v: Vector3) -> Vector3 {
        Vector3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Div for Vector3 {
    type Output = Vector3;
    fn div(self, v: Vector3) -> Vector3 {
        Vector3::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }
}

impl Add<f64> for Vector3 {
    type Output = Vector3;
    fn add(self, f: f64) -> Vector3 {
        Vector3::new(self.x + f, self.y + f, self.z + f)
    }
}

impl Sub<f64> for Vector3 {
    type Output = Vector3;
    fn sub(self, f: f64) -> Vector3 {
        Vector3::new(self.x - f, self.y - f, self.z - f)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, f: f64) -> Vector3 {
        Vector3::new(self.x * f, self.y * f, self.z * f)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, f: f64) -> Vector3 {
        Vector3::new(self.x / f, self.y / f, self.z / f)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, v: Vector3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, v: Vector3) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, v: Vector3) {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, v: Vector3) {
        self.x /= v.x;
        self.y /= v.y;
        self.z /= v.z;
    }
}

impl AddAssign<f64> for Vector3 {
    fn add_assign(&mut self, f: f64) {
        self.x += f;
        self.y += f;
        self.z += f;
    }
}

impl SubAssign<f64> for Vector3 {
    fn sub_assign(&mut self, f: f64) {
        self.x -= f;
        self.y -= f;
        self.z -= f;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, f: f64) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, f: f64) {
        self.x /= f;
        self.y /= f;
        self.z /= f;
    }
}

impl Add<Vector3> for f64 {
    type Output = Vector3;
    fn add(self, v: Vector3) -> Vector3 {
        Vector3::new(self + v.x, self + v.y, self + v.z)
    }
}

impl Sub<Vector3> for f64 {
    type Output = Vector3;
    fn sub(self, v: Vector3) -> Vector3 {
        Vector3::new(self - v.x, self - v.y, self - v.z)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl Div<Vector3> for f64 {
    type Output = Vector3;
    fn div(self, v: Vector3) -> Vector3 {
        Vector3::new(self / v.x, self / v.y, self / v.z)
    }
}

