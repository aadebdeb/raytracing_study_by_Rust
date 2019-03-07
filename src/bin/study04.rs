extern crate rayon;
extern crate image;
extern crate rand;
extern crate raytracing_study;

use std::sync::{ Arc, Mutex };

use rayon::prelude::*;
use image::{ Rgb, RgbImage };

use raytracing_study::{ vec3, Vector3, Transform, math, util };
use raytracing_study::{ Camera, Ray, Scene, PPrimitive, Geometry, TransformedPrimitive, Bvh, PShape, Rect };
use raytracing_study::{ LambertMaterial, FresnelBlendMaterial };

fn main() {
    // let (width, height) = (640, 480);
    let (width, height) = (1024, 768);

    let subpixel = 4;
    let inv_subpixel = 1.0 / (subpixel as f64);
    let samples = 100;

    let scene = create_scene();
    let camera = create_camera(width, height);
    let background = Background::new("./resources/GrandCanyon_C_YumaPoint/GCanyon_C_YumaPoint_8k.jpg");

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
                        subsum += render(&ray, &scene, &background, 0);
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

    image.save("./outputs/study04.jpg").unwrap();
}

fn render(ray: &Ray, scene: &Scene, background: &Background, depth: u32) -> Vector3 {
    if depth > 10 {
        return Vector3::zero();
    }
    scene.hit(ray).map_or(background.sample(ray.dir.norm()), |(isec, material)| {
        material.sample(&isec).map_or(Vector3::zero(), |bsdf| {
            if bsdf.pdf != 0.0 {
                let next_ray = Ray::new(isec.pos, bsdf.wi);
                let dot = isec.normal.dot(bsdf.wi).abs();
                bsdf.value * dot * render(&next_ray, scene, background, depth + 1) / bsdf.pdf
            } else {
                Vector3::zero()
            }
        })
    }) 
}

fn create_scene() -> Scene {
    let mut primitives: Vec<Box<PPrimitive>> = Vec::new();

    let bunny_mat = Arc::new(FresnelBlendMaterial::new(vec3(0.30, 0.05, 0.20), vec3(0.7, 0.7, 1.0), math::PI / 16.0));
    let bunny = util::load_stl("./resources/Bunny-LowPoly.stl").into_iter()
    // let bunny = util::load_obj("./resources/bunny.obj").into_iter()
        .map(|t| Box::new(Geometry::new(t, bunny_mat.clone())) as Box<PPrimitive>).collect();
    let bunny_prim = Bvh::new(bunny);
   let bunny_transform = Transform::rotate_x(-90.0).transform(&Transform::translate(-50.0, 0.0, 0.0));
    let bunny_prim = TransformedPrimitive::new(Box::new(bunny_prim), bunny_transform);
    primitives.push(Box::new(bunny_prim));

    let rect = Rect::new(200.0, 200.0);
    let rect_mat = LambertMaterial::new(vec3(0.7, 0.7, 0.7));
    let rect_prim = Geometry::new(Box::new(rect), Arc::new(rect_mat));
    primitives.push(Box::new(rect_prim));

    let scene = Scene::new(primitives);

    scene
}

fn create_camera(width: u32, height: u32) -> Camera {
    let cam_origin = vec3(-150.0, 100.0, 150.0);
    let cam_target = vec3(0.0, 50.0, 0.0);
    let cam_up = vec3(0.0, 1.0, 0.0);
    let camera = Camera::look_at(cam_origin, cam_target, cam_up, 60.0, (width as f64) / (height as f64));
    camera
}

struct Background {
    image: image::RgbImage,
}

impl Background {
    pub fn new(filename: &str) -> Background {
        let image = image::open(filename).unwrap().to_rgb();
        Background { image }
    }
    pub fn sample(&self, w: Vector3) -> Vector3 {
        let uv = math::sphere_uv(w);
        let pixel = self.image.get_pixel(
            (uv.0 * (self.image.width() as f64)) as u32,
            ((1.0 - uv.1) * (self.image.height() as f64)) as u32);
        vec3(
            math::gamma_to_linear(pixel[0] as f64 / 255.0),
            math::gamma_to_linear(pixel[1] as f64 / 255.0),
            math::gamma_to_linear(pixel[2] as f64 / 255.0)
        )
    }
}