use crate::math::Ray;
use crate::math::Vec3;
use crate::render::renderable::HitRecord;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray
}

pub trait Material {

    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}