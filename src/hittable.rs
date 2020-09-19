use crate::{material::*, ray::*, vec3::*};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Point::zeros(),
            normal: Vec3::zeros(),
            t: 0.0,
            front_face: false,
            material: Material::Lambertian(Color::zeros()),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if t_min < temp && temp < t_max {
                hit_record.t = temp;
                hit_record.point = ray.at(temp);
                let outward_normal = (hit_record.point - self.center) / self.radius;
                hit_record.set_face_normal(&ray, &outward_normal);
                hit_record.material = self.material;

                return true;
            }

            let temp = (-half_b + root) / a;
            if t_min < temp && temp < t_max {
                hit_record.t = temp;
                hit_record.point = ray.at(temp);
                let outward_normal = (hit_record.point - self.center) / self.radius;
                hit_record.set_face_normal(&ray, &outward_normal);
                hit_record.material = self.material;

                return true;
            }
        }

        return false;
    }
}
