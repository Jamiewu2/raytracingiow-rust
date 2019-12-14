use crate::math::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn origin(&self) -> Vec3 {
        return self.origin;
    }

    pub fn direction(&self) -> Vec3 {
        return self.direction;
    }

    pub fn point_at_distance(&self, t: f64) -> Vec3 {
        return self.origin + t * self.direction;
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vec3;
    use crate::math::Ray;

    #[test]
    fn test_point_at_distance() {
        let origin = Vec3::new(1.0,0.0,0.0);
        let direction = Vec3::new(1.0,1.0,1.0);
        let ray = Ray::new(origin, direction);

        let answer = Vec3::new(4.0, 3.0, 3.0);
        assert_eq!(ray.point_at_distance(3.0), answer);
    }
}
