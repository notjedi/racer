use crate::{ray::Ray, vector::Vec3};

const ASPECT_RATIO: f32 = 16.0 / 9.0;

const FOCAL_LENGTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;

const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;

const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const LOWER_LEFT: Vec3 = Vec3::new(
    ORIGIN.x() - (VIEWPORT_WIDTH / 2.0),
    ORIGIN.y() - (VIEWPORT_HEIGHT / 2.0),
    ORIGIN.z() - FOCAL_LENGTH,
);

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    image_width: usize,
    image_height: usize,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            origin: ORIGIN,
            lower_left: LOWER_LEFT,
            horizontal: HORIZONTAL,
            vertical: VERTICAL,
            image_width: IMAGE_WIDTH,
            image_height: IMAGE_HEIGHT,
        }
    }

    pub fn image_width(&self) -> usize {
        self.image_width
    }

    pub fn image_height(&self) -> usize {
        self.image_height
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        // TODO: why sub by ORIGIN?
        let direction =
            (&self.horizontal * u) + (&self.vertical * v) + (&self.lower_left - &self.origin);
        Ray::new(self.origin.clone(), direction)
    }
}
