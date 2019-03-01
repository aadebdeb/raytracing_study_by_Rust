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
    primitives: Vec<Primitive>,
}

impl Scene {
    pub fn new(primitives: Vec<Primitive>) -> Scene {
        Scene { primitives }
    }
    pub fn hit(&self, ray: &Ray) -> Option<(Intersection, Arc<PMaterial>)> {
        self.primitives.iter().fold(None, |res, primitive| {
            let tmax = res.as_ref().map_or(std::f64::MAX, |res| res.0.t);
            primitive.hit(ray, 1.0e-6, tmax).or(res)
        })
    }
}

pub struct Primitive {
    shape: Box<PShape>,
    material: Arc<PMaterial>,
}

impl Primitive {
    pub fn new(shape: Box<PShape>, material: Arc<PMaterial>) -> Primitive {
        Primitive { shape, material }
    }
    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(Intersection, Arc<PMaterial>)> {
        self.shape.hit(ray, tmin, tmax).map(|isec| (isec, self.material.clone()))
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

