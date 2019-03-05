use std::path::Path;

use crate::vec3;
use crate::Triangle;

pub fn load_obj(filename: &str) -> Vec<Box<Triangle>> {
    let (models, _) = tobj::load_obj(&Path::new(filename)).unwrap();
    let mesh = &models[0].mesh;
    let positions = &mesh.positions;
    let normals = &mesh.normals;
    let indices = &mesh.indices;
    indices.chunks(3).map(|idx| {
        let v0 = idx[0] as usize * 3;
        let v1 = idx[1] as usize * 3;
        let v2 = idx[2] as usize * 3;
        let positions = (
            vec3(positions[v0] as f64, positions[v0 + 1] as f64, positions[v0 + 2] as f64),
            vec3(positions[v1] as f64, positions[v1 + 1] as f64, positions[v1 + 2] as f64),
            vec3(positions[v2] as f64, positions[v2 + 1] as f64, positions[v2 + 2] as f64),
        );
        let normals = (
            vec3(normals[v0] as f64, normals[v0 + 1] as f64, normals[v0 + 2] as f64),
            vec3(normals[v1] as f64, normals[v1 + 1] as f64, normals[v1 + 2] as f64),
            vec3(normals[v2] as f64, normals[v2 + 1] as f64, normals[v2 + 2] as f64),
        );
        Box::new(Triangle::new(positions, normals))
    }).collect()
}

pub fn load_stl(filename: &str) -> Vec<Box<Triangle>> {
    let mut file = std::fs::OpenOptions::new().read(true).open(filename).unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();
    stl.faces.iter().map(|t| {
        let vs = t.vertices;
        let v0 = stl.vertices[vs[0]];
        let v1 = stl.vertices[vs[1]];
        let v2 = stl.vertices[vs[2]];

        let positions = (
            vec3(v0[0] as f64, v0[1] as f64, v0[2] as f64),
            vec3(v1[0] as f64, v1[1] as f64, v1[2] as f64),
            vec3(v2[0] as f64, v2[1] as f64, v2[2] as f64)
        );
        let normal = vec3(t.normal[0] as f64, t.normal[1] as f64, t.normal[2] as f64);
        let normals = (normal, normal, normal);
        Box::new(Triangle::new(positions, normals))
    }).collect()
}