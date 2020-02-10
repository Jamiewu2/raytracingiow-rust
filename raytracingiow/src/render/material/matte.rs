use crate::math::Vec3;
use rand::Rng;
//use crate::math::Ray;
//use crate::render::renderable::Renderable;

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




//impl Matte {
//    pub fn new() {
//
//    }
//
//    pub fn color_at(ray: &Ray, world: &Renderable) {
//
//    }
//}
