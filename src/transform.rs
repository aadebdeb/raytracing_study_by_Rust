use crate::{ vec3, Vector3, Matrix4 };
use crate::{ Ray, Aabb };

#[derive(Debug)]
pub struct Transform {
    mat: Matrix4,
    inv_mat: Matrix4,
}

fn min_from_array(array: &[Vector3]) -> Vector3 {
    array.iter().fold(Vector3::infinity(), |res, v| Vector3::min(res, *v))
}

fn max_from_array(array: &[Vector3]) -> Vector3 {
    array.iter().fold(Vector3::neg_infinity(), |res, v| Vector3::max(res, *v))
}

impl Transform {
    pub fn new(mat: Matrix4, inv_mat: Matrix4) -> Transform {
        Transform { mat, inv_mat }
    }
    pub fn identity() -> Transform {
        Transform::new(Matrix4::identity(), Matrix4::identity())
    }

    pub fn inverse(&self) -> Transform {
        Transform {
            mat: self.inv_mat.clone(),
            inv_mat: self.mat.clone(),
        }
    }

    pub fn translate(x: f64, y: f64, z: f64) -> Transform {
        let mat = Matrix4::new(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0
        );
        let inv_mat = Matrix4::new(
            1.0, 0.0, 0.0, -x,
            0.0, 1.0, 0.0, -y,
            0.0, 0.0, 1.0, -z,
            0.0, 0.0, 0.0, 1.0,
        );
        Transform::new(mat, inv_mat)
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Transform {
        let mat = Matrix4::new(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let inv_mat = Matrix4::new(
            1.0 / x, 0.0, 0.0, 0.0,
            0.0, 1.0 / y, 0.0, 0.0,
            0.0, 0.0, 1.0 / z, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        Transform::new(mat, inv_mat)
    }

    pub fn rotate_x(degree: f64) -> Transform {
        let r = degree.to_radians();
        let c = r.cos();
        let s = r.sin();
        let mat = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, c, -s, 0.0,
            0.0, s, c, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let inv_mat = mat.transpose();
        Transform::new(mat, inv_mat)
    }

    pub fn rotate_y(degree: f64) -> Transform {
        let r = degree.to_radians();
        let c = r.cos();
        let s = r.sin();
        let mat = Matrix4::new(
            c, 0.0, s, 0.0,
            0.0, 1.0, 0.0, 0.0,
            -s, 0.0, c, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let inv_mat = mat.transpose();
        Transform::new(mat, inv_mat)
    }

    pub fn rotate_z(degree: f64) -> Transform {
        let r = degree.to_radians();
        let c = r.cos();
        let s = r.sin();
        let mat = Matrix4::new(
            c, -s, 0.0, 0.0,
            s, c, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let inv_mat = mat.transpose();
        Transform::new(mat, inv_mat)
    }

    pub fn vector(&self, v: Vector3) -> Vector3 {
        Vector3::new(
            self.mat[0][0] * v.x + self.mat[0][1] * v.y + self.mat[0][2] * v.z,
            self.mat[1][0] * v.x + self.mat[1][1] * v.y + self.mat[1][2] * v.z,
            self.mat[2][0] * v.x + self.mat[2][1] * v.y + self.mat[2][2] * v.z
        )
    }

    pub fn point(&self, v: Vector3) -> Vector3 {
        Vector3::new(
            self.mat[0][0] * v.x + self.mat[0][1] * v.y + self.mat[0][2] * v.z + self.mat[0][3],
            self.mat[1][0] * v.x + self.mat[1][1] * v.y + self.mat[1][2] * v.z + self.mat[1][3],
            self.mat[2][0] * v.x + self.mat[2][1] * v.y + self.mat[2][2] * v.z + self.mat[2][3]
        )
    }

    pub fn normal(&self, v: Vector3) -> Vector3 {
        Vector3::new(
            self.inv_mat[0][0] * v.x + self.inv_mat[1][0] * v.y + self.inv_mat[2][0] * v.z,
            self.inv_mat[0][1] * v.x + self.inv_mat[1][1] * v.y + self.inv_mat[2][1] * v.z,
            self.inv_mat[0][2] * v.x + self.inv_mat[1][2] * v.y + self.inv_mat[2][2] * v.z
        )
    }

    pub fn ray(&self, ray: &Ray) -> Ray {
        Ray::new(self.point(ray.org), self.vector(ray.dir))
    }

    pub fn aabb(&self, aabb: &Aabb) -> Aabb {
        let p000 = self.point(vec3(aabb.min.x, aabb.min.y, aabb.min.z));
        let p001 = self.point(vec3(aabb.min.x, aabb.min.y, aabb.max.z));
        let p010 = self.point(vec3(aabb.min.x, aabb.max.y, aabb.min.z));
        let p011 = self.point(vec3(aabb.min.x, aabb.max.y, aabb.max.z));
        let p100 = self.point(vec3(aabb.max.x, aabb.min.y, aabb.min.z));
        let p101 = self.point(vec3(aabb.max.x, aabb.min.y, aabb.max.z));
        let p110 = self.point(vec3(aabb.max.x, aabb.max.y, aabb.min.z));
        let p111 = self.point(vec3(aabb.max.x, aabb.max.y, aabb.max.z));
        let array = [p000, p001, p010, p011, p100, p101, p110, p111];
        let min = min_from_array(&array);
        let max = max_from_array(&array);
        Aabb::new(min, max)
    }

    pub fn transform(&self, transform: &Transform) -> Transform {
        let mat = self.mat.mul(&transform.mat);
        let inv_mat = transform.inv_mat.mul(&self.inv_mat);
        Transform::new(mat, inv_mat)
    }
}
