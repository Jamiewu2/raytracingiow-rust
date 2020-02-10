use crate::math::Ray;
use crate::math::Vec3;
use crate::render::renderable::HitRecord;

pub trait Material {
    //todo add arguments (hitRecord?)
    //input ray as input, returns a vector of attentuation, and a scattered ray
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Vec3, Ray);
}