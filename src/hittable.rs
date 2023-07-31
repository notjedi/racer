use crate::{ray::Ray, vector::Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32, front_face: bool) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub struct Objects {
    objects: Vec<Box<dyn Hittable>>,
}

impl Objects {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut hit_record = None;

        self.objects.iter().for_each(|object| {
            if let Some(hit) = object.hit(ray, t_min, closest) {
                closest = hit.t;
                hit_record = Some(hit);
            }
        });

        hit_record
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin_centered = &ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let b = ray.direction.dot(&origin_centered) * 2.0;
        let c = origin_centered.length_squared() - (self.radius * self.radius);

        let discriminant = (b * b) - (a * c * 4.0);
        if discriminant < 0.0 {
            return None;
        }
        let discriminant = discriminant.sqrt();
        let roots = [
            (-b - discriminant) / (2.0 * a),
            (-b + discriminant) / (2.0 * a),
        ];

        let mut roots = roots.into_iter().filter(|&t| t > t_min && t < t_max);
        match roots.next() {
            Some(t) => {
                let point = ray.at(t);
                // TODO: why divide it by radius?
                let mut normal = (&point - &self.center) / self.radius;
                let front_face = ray.direction.dot(&normal) < 0.0;
                if !front_face {
                    normal = -normal;
                }
                // let normal = (&point - &self.center).normalize();
                // let normal = (normal * 0.5) + 0.5;
                Some(HitRecord::new(point, normal, t, front_face))
            }
            None => None,
        }
    }
}
