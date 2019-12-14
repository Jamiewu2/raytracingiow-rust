use crate::math::Vec3;
use crate::math::Ray;

pub struct HitRecord {
    //ray.point_at_distance(t) = position
    pub t: f64,
    pub position: Vec3,
    pub normal: Vec3
}

pub trait Renderable {
    //if the ray hits the renderable between t_min and t_max
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center,
            radius
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
                let position = ray.point_at_distance(t);
                let normal = (position - self.center).unit_vector();
                let hit_record = HitRecord {
                    t,
                    position,
                    normal,
                };

                return Option::Some(hit_record);
            }

            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if  t2 >= t_min && t2 <= t_max {
                let position = ray.point_at_distance(t);
                let normal = (position - self.center).unit_vector();
                let hit_record = HitRecord {
                    t: t2,
                    position,
                    normal,
                };

                return Option::Some(hit_record);
            }

            return Option::None;
        };
    }
}
