use crate::{ vec3, Vector3 };
use crate::Intersection;
use crate::math;

pub struct Bsdf {
    pub value: Vector3,
    pub wi: Vector3,
    pub pdf: f64,
}

pub type PMaterial = dyn Material + Sync + Send + 'static;

pub trait Material {
    fn bsdf(&self, wo: Vector3, wi: Vector3) -> Vector3;
    fn sample(&self, isec: &Intersection) -> Option<Bsdf>;
}

pub struct LambertMaterial {
    reflectance: Vector3,
}

impl LambertMaterial {
    pub fn new(reflectance: Vector3) -> LambertMaterial {
        LambertMaterial { reflectance }
    }
}

impl Material for LambertMaterial {
    fn bsdf(&self, wo: Vector3, wi: Vector3) -> Vector3 {
        self.reflectance / math::PI
    }
    fn sample(&self, isec: &Intersection) -> Option<Bsdf> {
        let (dir, pdf) = math::sample_random_cosine_dir();
        let wi = math::change_basis(dir, isec.normal);
        let value = self.bsdf(isec.wo, wi);
        Some(Bsdf { value, wi, pdf })
    }
}

pub struct SpecularReflectionMaterial {
    reflectance: Vector3,
}

impl SpecularReflectionMaterial {
    pub fn new(reflectance: Vector3) -> SpecularReflectionMaterial {
        SpecularReflectionMaterial { reflectance }
    }
}

impl Material for SpecularReflectionMaterial {
    fn bsdf(&self, _wo: Vector3, _wi: Vector3) -> Vector3 {
        Vector3::zero()
    }
    fn sample(&self, isec: &Intersection) -> Option<Bsdf> {
        let wi = math::reflect(-isec.wo, isec.normal);
        let value = self.reflectance / isec.normal.dot(wi).max(0.0);
        let pdf = 1.0;
        Some(Bsdf{ value, wi, pdf })
    }
}

pub struct SpecularTransmissionMaterial {
    transmittance: Vector3,
    ri: f64,
}

impl SpecularTransmissionMaterial {
    pub fn new(transmittance: Vector3, ri: f64) -> SpecularTransmissionMaterial {
        SpecularTransmissionMaterial { transmittance, ri }
    }
}

impl Material for SpecularTransmissionMaterial {
    fn bsdf(&self, _wo: Vector3, _wi: Vector3) -> Vector3 {
        Vector3::zero()
    }
    fn sample(&self, isec: &Intersection) -> Option<Bsdf> {
        let reflect = math::reflect(-isec.wo, isec.normal);
        let dot = isec.normal.dot(isec.wo);
        let (ri, normal, cosine) = if dot > 0.0 {
            (1.0 / self.ri, isec.normal, dot)
        } else {
            (self.ri, -isec.normal, -dot)
        };

        math::refract(isec.wo, normal, ri).map_or(Some(Bsdf {
            value: self.transmittance / cosine,
            wi: reflect,
            pdf: 1.0,
        }), |refract| {
            let fresnel = math::schlick_fresnel(cosine, ri);
            let r = rand::random::<f64>();
            if r < fresnel {
                Some(Bsdf {
                    value: fresnel * self.transmittance / cosine,
                    wi: reflect,
                    pdf: fresnel,
                })
            } else {
                Some(Bsdf {
                    value: (1.0 - fresnel) * self.transmittance / cosine,
                    wi: refract,
                    pdf: 1.0 - fresnel,
                })
            }
        })
    }
}