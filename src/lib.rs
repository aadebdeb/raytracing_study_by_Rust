mod vector3;
mod core;
mod bvh;
mod shape;
mod material;
pub mod math;
pub mod util;

pub use self::vector3::Vector3;
pub use self::core::{ Camera, Scene, Ray, Intersection, Primitive, PPrimitive, Geometry, Aabb };
pub use self::bvh::Bvh;
pub use self::shape::{ Shape, PShape, Sphere, Rect, Triangle };
pub use self::material::{ 
    Material, PMaterial, LambertMaterial,
    SpecularReflectionMaterial, SpecularTransmissionMaterial,
    MicrofacetReflectionMaterial};

pub fn vec3(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3::new(x, y, z)
}