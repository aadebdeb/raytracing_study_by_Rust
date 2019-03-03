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

pub struct Triangle {
    positions: (Vector3, Vector3, Vector3),
    normals: (Vector3, Vector3, Vector3),
}

impl Triangle {
    pub fn new(positions: (Vector3, Vector3, Vector3), normals: (Vector3, Vector3, Vector3)) -> Triangle {
        Triangle { positions, normals }
    }
}

fn intersect_triangle(ray: &Ray, positions: (Vector3, Vector3, Vector3)) -> Option<f64> {
    let e1 = positions.1 - positions.0;
    let e2 = positions.2 - positions.0;
    let alpha = ray.dir.cross(e2);
    let det = e1.dot(alpha);
    if det.abs() < 1e-6 {
        return None;
    }
    let inv_det = 1.0 / det;
    let r = ray.org - positions.0;

    let u = alpha.dot(r) * inv_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let beta = r.cross(e1);

    let v = ray.dir.dot(beta) * inv_det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = e2.dot(beta) * inv_det;
    if t < 0.0 {
        return None;
    }

    Some(t)
}

pub fn interpolate_normal(pos: Vector3, positions: (Vector3, Vector3, Vector3), normals: (Vector3, Vector3, Vector3)) -> Vector3 {
    let d0 = (positions.0 - pos).mag();
    let d1 = (positions.1 - pos).mag();
    let d2 = (positions.2 - pos).mag();
    ((d1 + d2) * normals.0 + (d2 + d0) * normals.1 + (d0 + d1) * normals.2).norm()
}

impl Shape for Triangle {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Intersection> {
        intersect_triangle(ray, self.positions)
            .filter(|t| *t > tmin && *t < tmax)
            .map(|t| {
                let pos = ray.at(t);
                let normal = interpolate_normal(pos, self.positions, self.normals);
                Intersection {
                    t,
                    wo: -ray.dir.norm(),
                    pos,
                    normal,
                }
            })
    }
}

pub struct Aggregate {
    shapes: Vec<Box<PShape>>,
}

impl Aggregate {
    pub fn new(shapes: Vec<Box<PShape>>) -> Aggregate {
        Aggregate {shapes}
    }
}

impl Shape for Aggregate {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Intersection> {
        self.shapes.iter().fold(None, |res, shape| {
            let t = res.as_ref().map_or(tmax, |isec| isec.t);
            shape.hit(ray, tmin, t).or(res)
        })
    }
}