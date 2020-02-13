use crate::math::Vec3;
use crate::render::material::material::Material;
use crate::math::Ray;
use crate::render::renderable::HitRecord;
use crate::render::material::random_in_unit_sphere;
use crate::render::material::material::ScatterRecord;

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let fuzz_val = if fuzz < 1_f64 { fuzz} else { 1_f64 };
        Self {
            albedo,
            fuzz: fuzz_val
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let n = ray.direction().dot(hit_record.normal);
        let reflected = ray.direction() - 2_f64 * n * hit_record.normal;
        let direction = reflected - hit_record.position;
        let fuzzed_direction = direction + self.fuzz * random_in_unit_sphere();

        if fuzzed_direction.dot(hit_record.normal) > 0_f64 {
            let attenuation = self.albedo;
            let scattered = Ray::new(hit_record.position, fuzzed_direction);

            return Some(
                ScatterRecord {
                    attenuation,
                    scattered
                }
            );
        } else {
            return None;
        }
    }
}
