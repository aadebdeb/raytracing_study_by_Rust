extern crate rayon;
extern crate image;
extern crate rand;
extern crate raytracing_study;

use std::sync::{ Arc, Mutex };

use rayon::prelude::*;
use image::{ Rgb, RgbImage };

use raytracing_study::{ vec3, Vector3, Transform, math, util };
use raytracing_study::{ Camera, Ray, Scene, PPrimitive, Geometry, TransformedPrimitive, Bvh, PShape, Rect, Sphere };
use raytracing_study::{ LambertMaterial, IlluminantMaterial };

fn main() {
    let (width, height) = (400, 400);
    // let (width, height) = (800, 800);

    let subpixel = 4;
    let inv_subpixel = 1.0 / (subpixel as f64);
    let samples = 1;

    let scene = create_scene();
    let camera = create_camera(width, height);

    let mut image = RgbImage::new(width, height);

    let progress = Mutex::new(0);

    image.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let x = *x as f64;
            let y = (height - *y - 1) as f64;

            let mut sum = Vector3::zero();
            for sx in 0..subpixel {
                for sy in 0..subpixel {
                    let mut subsum = Vector3::zero();
                    for _s in 0..samples {
                        let x = (x + (rand::random::<f64>() + (sx as f64)) * inv_subpixel) / (width as f64);
                        let y = (y + (rand::random::<f64>() + (sy as f64)) * inv_subpixel) / (height as f64);
                        let ray = camera.get_ray(x, y);
                        subsum += render(&ray, &scene, 0);
                    }
                    sum += subsum  / (samples as f64);
                }
            }
            sum /= (subpixel * subpixel) as f64;

            pixel[0] = (math::linear_to_gamma(math::clamp(sum.x, 0.0, 1.0)) * 255.99) as u8;
            pixel[1] = (math::linear_to_gamma(math::clamp(sum.y, 0.0, 1.0)) * 255.99) as u8;
            pixel[2] = (math::linear_to_gamma(math::clamp(sum.z, 0.0, 1.0)) * 255.99) as u8;
            let mut progress = progress.lock().unwrap();
            *progress += 1;
            if *progress % width == 0 {
                println!("progress of rendering: {:.2}%", 100.0 * (*progress as f64) / ((width * height) as f64));
            }
        });

    image.save("./outputs/study06.jpg").unwrap();
}

fn render(ray: &Ray, scene: &Scene, depth: u32) -> Vector3 {
    let max_bounce = 50;

    let mut ray = (*ray).clone();
    let mut result = Vector3::zero();
    let mut weight = Vector3::one();
    for bounce in 0..max_bounce {
        match scene.hit(&ray) {
            None => break,
            Some((isec, material)) => {
                result += weight * material.emit(&isec);
                match material.sample(&isec) {
                    None => break,
                    Some(bsdf) => {
                        let dot = isec.normal.dot(bsdf.wi).abs();
                        weight *= bsdf.value * dot / bsdf.pdf;
                        // russian roulette
                        let p = weight.x.max(weight.y.max(weight.z));
                        if rand::random::<f64>() > p {
                            break;
                        }
                        weight /= p;
                        ray = Ray::new(isec.pos, bsdf.wi);
                    }
                }
            }
        }
    }
    result

    // if depth > 10 {
    //     return Vector3::zero();
    // }
    // scene.hit(ray).map_or(vec3(0.0, 0.0, 0.0), |(isec, material)| {
    //     material.sample(&isec).map_or(material.emit(&isec), |bsdf| {
    //         if bsdf.pdf != 0.0 {
    //             let next_ray = Ray::new(isec.pos, bsdf.wi);
    //             let dot = isec.normal.dot(bsdf.wi).abs();
    //             material.emit(&isec) + bsdf.value * dot * render(&next_ray, scene, depth + 1) / bsdf.pdf
    //         } else {
    //             material.emit(&isec)
    //         }
    //     })
    // }) 
}

fn create_scene() -> Scene {
    let mut primitives: Vec<Box<PPrimitive>> = Vec::new();

    // cornel box
    let white_mat = Arc::new(LambertMaterial::new(vec3(0.95, 0.95, 0.95)));
    let red_mat = Arc::new(LambertMaterial::new(vec3(0.95, 0.1, 0.1)));
    let green_mat = Arc::new(LambertMaterial::new(vec3(0.1, 0.95, 0.1)));
    let light_mat = Arc::new(IlluminantMaterial::new(vec3(1.2, 1.2, 1.2)));

    let bottom_prim = Geometry::new(Box::new(Rect::new(10.0, 10.0)), white_mat.clone());

    let top_prim = Geometry::new(Box::new(Rect::new(10.0, 10.0)), white_mat.clone());
    let top_prim = TransformedPrimitive::new(Box::new(top_prim), Transform::rotate_x(-180.0).transform(&Transform::translate(0.0, 10.0, 0.0)));

    let far_prim = Geometry::new(Box::new(Rect::new(10.0, 10.0)), white_mat.clone());
    let far_prim = TransformedPrimitive::new(Box::new(far_prim), Transform::rotate_x(-90.0).transform(&Transform::translate(0.0, 5.0, 5.0)));

    let left_prim = Geometry::new(Box::new(Rect::new(10.0, 10.0)), green_mat.clone());
    let left_prim = TransformedPrimitive::new(Box::new(left_prim), Transform::rotate_z(-90.0).transform(&Transform::translate(-5.0, 5.0, 0.0)));

    let right_prim = Geometry::new(Box::new(Rect::new(10.0, 10.0)), red_mat.clone());
    let right_prim = TransformedPrimitive::new(Box::new(right_prim), Transform::rotate_z(90.0).transform(&Transform::translate(5.0, 5.0, 0.0)));

    let light_prim = Geometry::new(Box::new(Rect::new(3.0, 3.0)), light_mat.clone());
    let light_prim = TransformedPrimitive::new(Box::new(light_prim), Transform::rotate_x(-180.0).transform(&Transform::translate(0.0, 9.999, 0.0)));

    primitives.push(Box::new(bottom_prim));
    primitives.push(Box::new(top_prim));
    primitives.push(Box::new(far_prim));
    primitives.push(Box::new(left_prim));
    primitives.push(Box::new(right_prim));
    primitives.push(Box::new(light_prim));

    // let bunny_mat = Arc::new(LambertMaterial::new(vec3(0.3, 0.3, 0.3)));
    // let bunny = util::load_stl("./resources/Bunny-LowPoly.stl").into_iter()
    //     .map(|t| Box::new(Geometry::new(t, bunny_mat.clone())) as Box<PPrimitive>).collect();
    // let bunny_transform = Transform::rotate_x(-90.0)
    //     .transform(&Transform::translate(-50.0, 0.0, 0.0))
    //     .transform(&Transform::rotate_y(-135.0))
    //     .transform(&Transform::scale(0.05, 0.05, 0.05));
    // let bunny_prim = TransformedPrimitive::new(Box::new(Bvh::new(bunny)), bunny_transform);
    // primitives.push(Box::new(bunny_prim));

    let sphere_mat = Arc::new(LambertMaterial::new(vec3(0.3, 0.3, 0.3)));
    let sphere = Geometry::new(Box::new(Sphere::new(vec3(0.0, 2.5, 0.0), 2.5)), sphere_mat.clone());
    primitives.push(Box::new(sphere));

    let scene = Scene::new(primitives);

    scene
}

fn create_camera(width: u32, height: u32) -> Camera {
    let cam_origin = vec3(0.0, 5.0, -14.0);
    let cam_target = vec3(0.0, 5.0, 0.0);
    let cam_up = vec3(0.0, 1.0, 0.0);
    let camera = Camera::look_at(cam_origin, cam_target, cam_up, 60.0, (width as f64) / (height as f64));
    camera
}