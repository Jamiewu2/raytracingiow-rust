use crate::math::Ray;
use crate::math::Vec3;

pub struct HitRecord {
    //ray.point_at_distance(t) = position
    pub t: f64,
    pub position: Vec3,
    pub normal: Vec3,
}

pub trait Renderable {
    //if the ray hits the renderable between t_min and t_max
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<T: Renderable> Renderable for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut max = t_max;
        let mut curr: Option<HitRecord> = Option::None;

        for item in self {
            match item.hit(ray, t_min, max) {
                Option::Some(hit_record) => {
                    max = hit_record.t;
                    curr = Option::Some(hit_record);
                }
                Option::None => continue,
            }
        }

        return curr;
    }
}
