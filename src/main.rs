use std::{
    fs::File,
    io::{BufWriter, Write},
};

mod camera;
mod hittable;
mod ray;
mod vector;

use camera::Camera;
use hittable::{Objects, Sphere};
use rand::{distributions::Uniform, Rng};
use ray::Ray;
use vector::Vec3;

const SAMPLES_PER_PIXEL: usize = 100;
const WHITE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const BLUE: Vec3 = Vec3::new(0.0, 0.0, 1.0);

fn get_color_of_ray(ray: &Ray, objects: &Objects) -> Vec3 {
    let hit_record = objects.hit(ray, 0.0, f32::MAX);
    match hit_record {
        Some(hit_record) => (&hit_record.normal * 0.5) + 0.5,
        None => {
            let norm_direction = ray.direction.normalize();
            let t = (norm_direction.y() + 1.0) * 0.5; // scale to be between 0.0 and 1.0
            WHITE * (1.0 - t) + BLUE * t
        }
    }
}

fn gen_ppm_file() {
    let mut objects = Objects::new();
    objects.add(Box::new(Sphere::new(Vec3::from((0.0, 0.0, -1.0)), 0.5)));
    objects.add(Box::new(Sphere::new(Vec3::from((0.0, -50.0, -1.0)), 100.0)));

    let camera = Camera::new();
    let image_width = camera.image_width();
    let image_height = camera.image_height();

    let mut rng = rand::thread_rng();
    let dist = Uniform::new(0.0 as f32, 1.0);

    let file = File::create("img.ppm").unwrap();
    let mut stream = BufWriter::new(file);

    stream
        .write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())
        .unwrap();

    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let mut color_accum = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (j as f32 + rng.sample(dist)) / (image_width - 1) as f32;
                let v = (i as f32 + rng.sample(dist)) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                let color = get_color_of_ray(&ray, &objects);
                color_accum = color_accum + color;
            }
            color_accum = color_accum / SAMPLES_PER_PIXEL as f32;
            color_accum = color_accum * 255.0;

            stream
                .write_all(
                    format!(
                        "{} {} {}\n",
                        color_accum.r() as i32,
                        color_accum.g() as i32,
                        color_accum.b() as i32
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
    }
    stream.flush().unwrap();
}

fn main() {
    gen_ppm_file();
}
