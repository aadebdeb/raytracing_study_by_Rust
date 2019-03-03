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
    fn bsdf(&self, isec: &Intersection, wi: Vector3) -> Vector3;
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
    fn bsdf(&self, _isec: &Intersection, _wi: Vector3) -> Vector3 {
        self.reflectance / math::PI
    }
    fn sample(&self, isec: &Intersection) -> Option<Bsdf> {
        let (dir, pdf) = math::sample_random_cosine_dir();
        let wi = math::change_basis(dir, isec.normal);
        let value = self.bsdf(isec, wi);
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
    fn bsdf(&self, _isec: &Intersection, _wi: Vector3) -> Vector3 {
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
    fn bsdf(&self, _isec: &Intersection, _wi: Vector3) -> Vector3 {
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

pub struct MicrofacetReflectionMaterial {
    reflectance: Vector3,
    roughness: f64,

}

impl MicrofacetReflectionMaterial {
    pub fn new(reflectance: Vector3, roughness: f64) -> MicrofacetReflectionMaterial {
        MicrofacetReflectionMaterial { reflectance, roughness }
    }
}

fn ggx_distribution(wh: Vector3, normal: Vector3, roughness: f64) -> f64 {
    let rough2 = math::pow2(roughness);
    let dot_nh = normal.dot(wh);
    rough2 / (math::PI * math::pow2(1.0 -(1.0 - rough2) * math::pow2(dot_nh)))
}

fn ggx_g(wo: Vector3, wi: Vector3, normal: Vector3, roughness: f64) -> f64 {
    1.0 / (1.0 + ggx_g1(wo, normal, roughness) + ggx_g1(wi, normal, roughness))
}

fn ggx_g1(v: Vector3, normal: Vector3, roughness: f64) -> f64 {
    let rough2 = math::pow2(roughness);
    let dot2 = math::pow2(normal.dot(v));
    0.5 * (-1.0 + (1.0 + rough2 * (1.0 / dot2 - 1.0)).sqrt())
}

impl Material for MicrofacetReflectionMaterial {
    fn bsdf(&self, isec: &Intersection, wi: Vector3) -> Vector3 {
        let wh = (isec.wo + wi).norm();
        let d = ggx_distribution(wh, isec.normal, self.roughness);
        let g = ggx_g(isec.wo, wi, isec.normal, self.roughness);
        let f = 1.0;
        let dot_no = isec.normal.dot(isec.wo);
        let dot_ni = isec.normal.dot(wi);
        self.reflectance * d * g * f / (4.0 * dot_no * dot_ni)
    }
    fn sample(&self, isec: &Intersection) -> Option<Bsdf> {
        let (dir, pdf) = math::sample_random_cosine_dir();
        let wi = math::change_basis(dir, isec.normal);
        let value = self.bsdf(isec, wi);
        Some(Bsdf { value, wi, pdf })
    }
}

// pub struct MicrofacetTransmissionMaterial {
//     transmittance: Vector3,
//     ri: f64,
//     roughness: f64,
// }

// impl MicrofacetTransmissionMaterial {
//     pub fn new(transmittance: Vector3, ri: f64, roughness: f64) -> MicrofacetTransmissionMaterial {
//         MicrofacetTransmissionMaterial { transmittance, ri, roughness }
//     }
// }

// impl Material for MicrofacetTransmissionMaterial {
//     fn bsdf(&self, isec: &Intersection, wi: Vector3) -> Vector3 {
//         let dot = isec.normal.dot(isec.wo);
//         let (ri, normal) = if dot > 0.0 {
//             (self.ri, isec.normal)
//         } else {
//             (1.0 / self.ri, -isec.normal)
//         };
//         let wh = (isec.wo + ri * wi).norm();

//         let dot_no = normal.dot(isec.wo);
//         let dot_ni = normal.dot(wi);
//         let dot_ho = wh.dot(isec.wo);
//         let dot_hi = wh.dot(wi);

//         let d = ggx_distribution(wh, normal, self.roughness);
//         let g = ggx_g(isec.wo, wi, normal, self.roughness);
//         let f = math::schlick_fresnel(dot_no, ri);

//         self.transmittance * math::pow2(self.ri) * d * g * (1.0 - f) * dot_ho.abs() * dot_hi.abs()
//             / (math::pow2(dot_ho + self.ri * dot_hi) * dot_no * dot_ni)
//     }
//     fn sample(&self, isec: &Intersection) -> Option<Bsdf> {
//         let (dir, pdf) = math::sample_random_cosine_dir();
//         let normal = if isec.normal.dot(isec.wo) > 0.0 { isec.normal } else { -isec.normal };
//         let wi = math::change_basis(dir, normal);
//         let value = self.bsdf(isec, wi);
//         Some(Bsdf { value, wi, pdf })
//     }
// }