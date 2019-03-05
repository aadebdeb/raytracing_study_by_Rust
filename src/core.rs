use std::sync::Arc;

use crate::Vector3;
use crate::PShape;
use crate::PMaterial;

pub struct Ray {
    pub org: Vector3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(org: Vector3, dir: Vector3) -> Ray {
        Ray { org, dir }
    }
    pub fn at(&self, t: f64) -> Vector3 {
        self.org + t * self.dir
    }
}

pub struct Intersection {
    pub t: f64,
    pub wo: Vector3,
    pub pos: Vector3,
    pub normal: Vector3,
}

pub struct Scene {
    primitives: Vec<Box<PPrimitive>>,
}

impl Scene {
    pub fn new(primitives: Vec<Box<PPrimitive>>) -> Scene {
        Scene { primitives }
    }
    pub fn hit(&self, ray: &Ray) -> Option<(Intersection, Arc<PMaterial>)> {
        self.primitives.iter().fold(None, |res, primitive| {
            let tmax = res.as_ref().map_or(std::f64::MAX, |res| res.0.t);
            primitive.hit(ray, 1.0e-6, tmax).or(res)
        })
    }
}

pub trait Primitive {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(Intersection, Arc<PMaterial>)>;
    fn aabb(&self) -> &Aabb;
}

pub type PPrimitive = dyn Primitive + Sync + 'static;

pub struct Geometry {
    shape: Box<PShape>,
    material: Arc<PMaterial>,
}

impl Geometry {
    pub fn new(shape: Box<PShape>, material: Arc<PMaterial>) -> Geometry {
        Geometry { shape, material }
    }
}

impl Primitive for Geometry {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(Intersection, Arc<PMaterial>)> {
        self.shape.hit(ray, tmin, tmax).map(|isec| (isec, self.material.clone()))
    }
    fn aabb(&self) -> &Aabb {
        self.shape.aabb()
    }
}

pub struct Camera {
    origin: Vector3,
    basis: (Vector3, Vector3, Vector3),
}

impl Camera {
    pub fn look_at(origin: Vector3, target: Vector3, vup: Vector3, vfov: f64, aspect: f64) -> Camera {
        let hh = (vfov.to_radians() / 2.0).tan();
        let hw = aspect * hh;
        let front = (target - origin).norm();
        let right = front.cross(vup).norm();
        let up = right.cross(front);
        let basis = (hw * right, hh * up, front);
        Camera { origin, basis }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        assert!(u >= 0.0 && u <= 1.0 && v >= 0.0 && v <= 1.0);
        let u = 2.0 * u - 1.0;
        let v = 2.0 * v - 1.0;
        Ray::new(self.origin, self.basis.0 * u + self.basis.1 * v + self.basis.2)
    }
}

pub struct Aabb(pub Vector3, pub Vector3);

impl Aabb {
    pub fn area(&self) -> f64 {
        let x = self.1.x - self.0.x;
        let y = self.1.y - self.0.y;
        let z = self.1.z - self.0.z;
        2.0 * (x * y + y * z + z * x)
    }
    pub fn merge(&self, other: &Aabb) -> Aabb {
        Aabb(Vector3::min(self.0, other.0), Vector3::max(self.1, other.1))
    }
    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> bool {
        for i in 0..3 {
            let t0 = (self.0[i] - ray.org[i]) / ray.dir[i];
            let t1 = (self.1[i] - ray.org[i]) / ray.dir[i];
            let tmin = t0.min(t1).max(tmin);
            let tmax = t0.max(t1).min(tmax);
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}
