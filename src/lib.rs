mod vector3;
mod core;
mod shape;
mod material;
pub mod math;

pub use self::vector3::Vector3;
pub use self::core::{ Camera, Scene, Ray, Intersection, Primitive };
pub use self::shape::{ Shape, PShape, Sphere, Rect };
pub use self::material::{ Material, PMaterial, LambertMaterial, SpecularReflectionMaterial, SpecularTransmissionMaterial };

pub fn vec3(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3::new(x, y, z)
}