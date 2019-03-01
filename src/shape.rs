use crate::{ vec3, Vector3 };
use crate::math;
use crate::{ Ray, Intersection };

pub trait Shape {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Intersection>;
}

pub type PShape = dyn Shape + Sync + 'static;

pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Intersection> {
        let oc = ray.org - self.center;
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        math::solve_quadratic_equation(a, b, c).map_or(None, |(t1, t2)| {
            if t1 > tmin && t1 < tmax {
                let pos = ray.at(t1);
                Some(Intersection {
                    t: t1,
                    wo: -ray.dir.norm(),
                    pos,
                    normal: (pos - self.center).norm(),
                })
            } else if t2 > tmin && t2 < tmax {
                let pos = ray.at(t2);
                Some(Intersection {
                    t: t2,
                    wo: -ray.dir.norm(),
                    pos,
                    normal: (pos - self.center).norm(),
                })
            } else {
                None
            }
        })
    }
}


pub struct Rect {
    width: f64,
    height: f64,
}

impl Rect {
    pub fn new(width: f64, height: f64) -> Rect {
        Rect { width, height }
    }
}

impl Shape for Rect {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Intersection> {
        if ray.dir.x.abs() == 0.0 {
            return None;
        }
        let t = -ray.org.y / ray.dir.y;
        if t <= tmin || t >= tmax {
            return None;
        }
        let pos = ray.at(t);
        let hw = self.width * 0.5;
        let hh = self.height * 0.5;
        if pos.x > -hw && pos.x < hw && pos.z > -hh && pos.z < hh {
            Some(Intersection {
                t,
                pos,
                wo: -ray.dir.norm(),
                normal: vec3(0.0, 1.0, 0.0),
            })
        } else {
            None
        }
    }
}