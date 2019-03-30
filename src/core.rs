use std::sync::Arc;

use crate::Vector3;
use crate::PShape;
use crate::PMaterial;
use crate::Transform;

#[derive(Debug, Clone)]
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
    fn sample(&self) -> (Vector3, f64) {
        panic!("sample method has not implemented")
    }
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
    fn sample(&self) -> (Vector3, f64) {
        self.shape.sample()
    }
}

pub struct Camera {
    origin: Vector3,
    basis: (Vector3, Vector3, Vector3),
}

pub struct TransformedPrimitive {
    primitive: Box<PPrimitive>,
    transform: Transform,
    inv_transform: Transform,
    aabb: Aabb,
}

impl TransformedPrimitive {
    pub fn new(primitive: Box<PPrimitive>, transform: Transform) -> TransformedPrimitive {
        let inv_transform = transform.inverse();
        let aabb = transform.aabb(primitive.aabb());
        TransformedPrimitive { primitive, transform, inv_transform, aabb }
    }
}

impl Primitive for TransformedPrimitive {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(Intersection, Arc<PMaterial>)> {
        let inv_ray = self.inv_transform.ray(ray);
        self.primitive.hit(&inv_ray, tmin, tmax).map(|(isec, material)| {
            (Intersection {
                t: isec.t,
                wo: self.transform.vector(isec.wo),
                pos: self.transform.point(isec.pos),
                normal: self.transform.normal(isec.normal),
            }, material)
        })
    }
    fn aabb(&self) -> &Aabb {
        &self.aabb
    }
    fn sample(&self) -> (Vector3, f64) {
        let (pos, prob) = self.primitive.sample();
        (self.transform.point(pos), prob)
    }
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

pub struct Aabb {
    pub min: Vector3,
    pub max: Vector3,
    center: Vector3,
}

impl Aabb {
    pub fn new(min: Vector3, max: Vector3) -> Aabb {
        let center = (min + max) * 0.5;
        Aabb { min, max, center }
    }
    pub fn area(&self) -> f64 {
        let x = self.max.x - self.min.x;
        let y = self.max.y - self.min.y;
        let z = self.max.z - self.min.z;
        2.0 * (x * y + y * z + z * x)
    }
    pub fn merge(&self, other: &Aabb) -> Aabb {
        Aabb::new(Vector3::min(self.min, other.min), Vector3::max(self.max, other.max))
    }
    pub fn center(&self) -> Vector3 {
        self.center
    }
    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> bool {
        let mut tmin = tmin;
        let mut tmax = tmax;
        for i in 0..3 {
            let t0 = (self.min[i] - ray.org[i]) / ray.dir[i];
            let t1 = (self.max[i] - ray.org[i]) / ray.dir[i];
            tmin = t0.min(t1).max(tmin);
            tmax = t0.max(t1).min(tmax);
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}
