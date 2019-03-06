use std::ops::{ Mul, Index, IndexMut };

#[derive(Debug, Clone)]
pub struct Matrix4 {
    elements: [[f64; 4]; 4],
}

impl Matrix4 {
    pub fn new(
        e00: f64, e01: f64, e02: f64, e03: f64,
        e10: f64, e11: f64, e12: f64, e13: f64,
        e20: f64, e21: f64, e22: f64, e23: f64,
        e30: f64, e31: f64, e32: f64, e33: f64) -> Matrix4
    {
        Matrix4 {
            elements: [
                [e00, e01, e02, e03],
                [e10, e11, e12, e13],
                [e20, e21, e22, e23],
                [e30, e31, e32, e33],
            ]
        }
    }

    pub fn identity() -> Matrix4 {
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn transpose(&self) -> Matrix4 {
        Matrix4::new(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3],
        )
    }

    pub fn mul(&self, m: &Matrix4) -> Matrix4 {
        Matrix4::new(
            m[0][0] * self[0][0] + m[0][1] * self[1][0] + m[0][2] * self[2][0] + m[0][3] * self[3][0],
            m[0][0] * self[0][1] + m[0][1] * self[1][1] + m[0][2] * self[2][1] + m[0][3] * self[3][1],
            m[0][0] * self[0][2] + m[0][1] * self[1][2] + m[0][2] * self[2][2] + m[0][3] * self[3][2],
            m[0][0] * self[0][3] + m[0][1] * self[1][3] + m[0][2] * self[2][3] + m[0][3] * self[3][3],

            m[1][0] * self[0][0] + m[1][1] * self[1][0] + m[1][2] * self[2][0] + m[1][3] * self[3][0],
            m[1][0] * self[0][1] + m[1][1] * self[1][1] + m[1][2] * self[2][1] + m[1][3] * self[3][1],
            m[1][0] * self[0][2] + m[1][1] * self[1][2] + m[1][2] * self[2][2] + m[1][3] * self[3][2],
            m[1][0] * self[0][3] + m[1][1] * self[1][3] + m[1][2] * self[2][3] + m[1][3] * self[3][3],

            m[2][0] * self[0][0] + m[2][1] * self[1][0] + m[2][2] * self[2][0] + m[2][3] * self[3][0],
            m[2][0] * self[0][1] + m[2][1] * self[1][1] + m[2][2] * self[2][1] + m[2][3] * self[3][1],
            m[2][0] * self[0][2] + m[2][1] * self[1][2] + m[2][2] * self[2][2] + m[2][3] * self[3][2],
            m[2][0] * self[0][3] + m[2][1] * self[1][3] + m[2][2] * self[2][3] + m[2][3] * self[3][3],

            m[3][0] * self[0][0] + m[3][1] * self[1][0] + m[3][2] * self[2][0] + m[3][3] * self[3][0],
            m[3][0] * self[0][1] + m[3][1] * self[1][1] + m[3][2] * self[2][1] + m[3][3] * self[3][1],
            m[3][0] * self[0][2] + m[3][1] * self[1][2] + m[3][2] * self[2][2] + m[3][3] * self[3][2],
            m[3][0] * self[0][3] + m[3][1] * self[1][3] + m[3][2] * self[2][3] + m[3][3] * self[3][3]
        )
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f64];
    fn index(&self, index: usize) -> &[f64] {
        &self.elements[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut [f64] {
        &mut self.elements[index]
    }
}