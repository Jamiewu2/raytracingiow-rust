use crate::math::Vec3;
use crate::math::Ray;

pub struct Camera {
    bottom_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3
}

impl Camera {
    pub fn new(bottom_left: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Camera {
        Camera {
            bottom_left,
            horizontal,
            vertical,
            origin
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(self.origin, self.bottom_left + u * self.horizontal + v * self.vertical - self.origin);
    }
}
