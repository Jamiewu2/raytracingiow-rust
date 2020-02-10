
use crate::math::Vec3;
use crate::math::Ray;
use crate::render::renderable::HitRecord;
use crate::render::renderable::Renderable;
use crate::render::material::Material;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Self {
        Self { center, radius, material }
    }

    fn create_hit_record(&self, ray: &Ray, t: f64) -> HitRecord {
        let position = ray.point_at_distance(t);
        let normal = (position - self.center).unit_vector();
        HitRecord {
            t,
            position,
            normal,
            material: &*self.material
        }
    }
}

impl Renderable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ac = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(ac);
        let c = ac.dot(ac) - self.radius * self.radius;

        let discriminant = b.powi(2) - 4_f64 * a * c;

        if discriminant < 0.0 {
            return Option::None;
        } else {
            //quadratic formula
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t >= t_min && t <= t_max {
                let hit_record = self.create_hit_record(ray, t);
                return Option::Some(hit_record);
            }

            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if t2 >= t_min && t2 <= t_max {
                let hit_record = self.create_hit_record(ray, t);
                return Option::Some(hit_record);
            }

            return Option::None;
        };
    }
}
