use std::{
    fs::File,
    io::{BufWriter, Write},
};

mod ray;
mod vector;

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

const CIRCLE_CENTER: Vec3 = Vec3::new(0.0, 0.0, -1.0);
const CIRCLE_RADIUS: f32 = 0.5;

const WHITE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const BLUE: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const RED: Vec3 = Vec3::new(1.0, 0.0, 0.0);

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> (bool, [f32; 2]) {
    let origin_centered = &ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = ray.direction.dot(&origin_centered) * 2.0;
    let c = -(radius * radius) + origin_centered.dot(&origin_centered);
    let discriminant = ((b * b) - (a * c * 4.0)).sqrt();
    (
        discriminant > 0.0,
        [
            (-b + discriminant) / (2.0 * a),
            (-b - discriminant) / (2.0 * a),
        ],
    )
}

fn get_color_of_ray(ray: Ray) -> Vec3 {
    let (does_hit, t) = hit_sphere(&CIRCLE_CENTER, CIRCLE_RADIUS, &ray);
    if does_hit {
        let point_1 = ray.at(t[1]);
        let normal = (&point_1 - &CIRCLE_CENTER).normalize();
        let normal = (normal * 0.5) + 0.5;
        // let normal = (normal + 1.0) * 0.5;
        return normal;
    }
    let norm_direction = ray.direction.normalize();
    let t = (norm_direction.y() + 1.0) * 0.5; // scale to be between 0.0 and 1.0
    WHITE * (1.0 - t) + BLUE * t
}

fn gen_ppm_file() {
    let file = File::create("img.ppm").unwrap();
    let mut stream = BufWriter::new(file);

    stream
        .write_all(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes())
        .unwrap();

    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let x = j as f32 / (IMAGE_WIDTH - 1) as f32;
            let y = i as f32 / (IMAGE_HEIGHT - 1) as f32;

            // TODO: why sub by ORIGIN?
            let direction = LOWER_LEFT + (HORIZONTAL * x) + (VERTICAL * y) - ORIGIN;
            let ray = Ray::new(ORIGIN, direction);

            // let color = get_color_of_ray(ray) * 255.999;
            let color = get_color_of_ray(ray);
            let color = color * 255.999;
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
