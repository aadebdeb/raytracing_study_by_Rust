use std::sync::Arc;

use rand::random;

use crate::Vector3;
use crate::{ Ray, Primitive, PPrimitive, Intersection, PMaterial, Aabb };

fn merge_aabb(primitives: &Vec<Box<PPrimitive>>) -> Aabb {
    let min = Vector3::infinity();
    let max = Vector3::neg_infinity();
    primitives.iter().fold(Aabb(min, max), |aabb, primitive| {
        aabb.merge(primitive.aabb())
    })
}


enum BvhNode {
    Branch(Box<BvhNode>, Box<BvhNode>, Aabb),
    Leaf(Box<PPrimitive>),
}

impl BvhNode {
    pub fn new(mut primitives: Vec<Box<PPrimitive>>) -> BvhNode {
        assert!(primitives.len() > 0);

        if primitives.len() == 1 {
            return BvhNode::Leaf(primitives.remove(0));
        }

        let aabb = merge_aabb(&primitives);

        let axis = (random::<f64>() * 3.0) as usize;
        primitives.sort_by(|a, b| a.aabb().0[axis].partial_cmp(&b.aabb().0[axis]).unwrap());

        let primitives2 = primitives.split_off(primitives.len() / 2);

        let child1 = BvhNode::new(primitives);
        let child2 = BvhNode::new(primitives2);

        BvhNode::Branch(Box::new(child1), Box::new(child2), aabb)
    }

    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(Intersection, Arc<PMaterial>)> {
        match self {
            BvhNode::Leaf(primitive) => primitive.hit(ray, tmin, tmax),
            BvhNode::Branch(child1, child2, aabb) => {
                if !aabb.hit(ray, tmin, tmax) {
                    None
                } else {
                    let hit1 = child1.hit(ray, tmin, tmax);
                    let hit2 = child2.hit(ray, tmin, tmax);
                    match (hit1, hit2) {
                        (None, None) => None,
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(b),
                        (Some(a), Some(b)) => Some(if a.0.t < b.0.t { a } else { b }),
                    }
                }
            },
        }
    }

    pub fn aabb(&self) -> &Aabb {
        match self {
            BvhNode::Branch(_, _, aabb) => &aabb,
            BvhNode::Leaf(primitive) => primitive.aabb(),
        }
    }
}

pub struct Bvh {
    root: BvhNode,
}

impl Bvh {
    pub fn new(primitives: Vec<Box<PPrimitive>>) -> Bvh {
        Bvh { root: BvhNode::new(primitives) }
    }
}

impl Primitive for Bvh {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(Intersection, Arc<PMaterial>)> {
        self.root.hit(ray, tmin, tmax)
    }
    fn aabb(&self) -> &Aabb {
        self.root.aabb()
    }
}