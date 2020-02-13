use crate::math::Vec3;
use rand::Rng;
use crate::render::material::material::Material;
use crate::math::Ray;
use crate::render::renderable::HitRecord;
use crate::render::material::material::ScatterRecord;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let i: f64 = rng.gen();
        let j: f64 = rng.gen();
        let k: f64 = rng.gen();

        let v: Vec3 = 2_f64 * Vec3::new(i, j, k) - Vec3::new(1.0, 1.0, 1.0);

        if v.length_squared() <= 1_f64 {
            return v;
        }
    }
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let target = hit_record.position + hit_record.normal + random_in_unit_sphere();
        let direction = target - hit_record.position;

        let scattered = Ray::new(hit_record.position, direction);
        let attenuation = self.albedo;

        return Some(
            ScatterRecord {
                attenuation,
                scattered
            }
        );
    }
}
