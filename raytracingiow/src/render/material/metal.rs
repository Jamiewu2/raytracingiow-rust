use crate::math::Vec3;
use crate::render::material::material::Material;
use crate::math::Ray;
use crate::render::renderable::HitRecord;

pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Vec3, Ray) {
        let n = ray.direction().dot(hit_record.normal);
        let reflected = ray.direction() - 2_f64 * n * hit_record.normal;
        let direction = reflected - hit_record.position;

        let scattered = Ray::new(hit_record.position, direction);
        let attentuation = self.albedo;

        return (attentuation, scattered);
    }
}
