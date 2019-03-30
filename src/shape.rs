use crate::{ vec3, Vector3 };
use crate::math;
use crate::{ Ray, Intersection, Aabb };

pub trait Shape {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Intersection>;
    fn aabb(&self) -> &Aabb;
    fn sample(&self) -> (Vector3, f64) {
        panic!("sample method has not implemented");
    }
}

pub type PShape = dyn Shape + Sync + 'static;

pub struct Sphere {
    center: Vector3,
    radius: f64,
    aabb: Aabb,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Sphere {
        let aabb = Aabb::new(center - radius, center + radius);
        Sphere { center, radius, aabb }
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
    fn aabb(&self) -> &Aabb {
        &self.aabb
    }
}


pub struct Rect {
    width: f64,
    height: f64,
    aabb: Aabb,
}

impl Rect {
    pub fn new(width: f64, height: f64) -> Rect {
        let hw = 0.5 * width;
        let hh = 0.5 * height;
        let aabb = Aabb::new(vec3(-hw, -0.01, -hh), vec3(hw, 0.01, hh));
        Rect { width, height, aabb }
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
    fn aabb(&self) -> &Aabb {
        &self.aabb
    }
    fn sample(&self) -> (Vector3, f64) {
        let hw = 0.5 * self.width;
        let hh = 0.5 * self.height;
        let pos = vec3(rand::random::<f64>() * self.width - hw, 0.0, rand::random::<f64>() * self.height - hh);
        let prob = 1.0 / (self.width * self.height);
        (pos, prob)
    }
}

pub struct Triangle {
    positions: (Vector3, Vector3, Vector3),
    normals: (Vector3, Vector3, Vector3),
    aabb: Aabb,
}

impl Triangle {
    pub fn new(positions: (Vector3, Vector3, Vector3), normals: (Vector3, Vector3, Vector3)) -> Triangle {
        let aabb = Aabb::new(
            Vector3::min(positions.0, Vector3::min(positions.1, positions.2)),
            Vector3::max(positions.0, Vector3::max(positions.1, positions.2))
        );
        Triangle { positions, normals, aabb }
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
    fn aabb(&self) -> &Aabb {
        &self.aabb
    }
}

// pub struct Aggregate {
//     shapes: Vec<Box<PShape>>,
//     aabb: Aabb,
// }

// impl Aggregate {
//     pub fn new(shapes: Vec<Box<PShape>>) -> Aggregate {
//         Aggregate {shapes}
//     }
// }

// impl Shape for Aggregate {
//     fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Intersection> {
//         self.shapes.iter().fold(None, |res, shape| {
//             let t = res.as_ref().map_or(tmax, |isec| isec.t);
//             shape.hit(ray, tmin, t).or(res)
//         })
//     }
// }