use std::{
    fs::File,
    io::{BufWriter, Write},
};

mod hittable;
mod ray;
mod vector;

use hittable::{Objects, Sphere};
use ray::Ray;
use vector::Vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;

const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;

const FOCAL_LENGTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;

const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const LOWER_LEFT: Vec3 = Vec3::new(
    ORIGIN.x() - (VIEWPORT_WIDTH / 2.0),
    ORIGIN.y() - (VIEWPORT_HEIGHT / 2.0),
    ORIGIN.z() - FOCAL_LENGTH,
);

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

    let file = File::create("img.ppm").unwrap();
    let mut stream = BufWriter::new(file);

    stream
        .write_all(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes())
        .unwrap();

    for i in (0..IMAGE_HEIGHT).rev() {
        for j in 0..IMAGE_WIDTH {
            let x = j as f32 / (IMAGE_WIDTH - 1) as f32;
            let y = i as f32 / (IMAGE_HEIGHT - 1) as f32;

            // TODO: why sub by ORIGIN?
            let direction = LOWER_LEFT + (HORIZONTAL * x) + (VERTICAL * y) - ORIGIN;
            let ray = Ray::new(ORIGIN, direction);

            let color = get_color_of_ray(&ray, &objects);
            let color = color * 255.0;
            stream
                .write_all(
                    format!(
                        "{} {} {}\n",
                        color.r() as i32,
                        color.g() as i32,
                        color.b() as i32
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
