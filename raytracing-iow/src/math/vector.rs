use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Self {
            x: e1,
            y: e2,
            z: e3,
        }
    }

    pub fn x(&self) -> f64 {
        return self.x;
    }
    pub fn y(&self) -> f64 {
        return self.y;
    }
    pub fn z(&self) -> f64 {
        return self.z;
    }

    pub fn r(&self) -> f64 {
        return self.x;
    }
    pub fn g(&self) -> f64 {
        return self.y;
    }
    pub fn b(&self) -> f64 {
        return self.z;
    }

    pub fn length(&self) -> f64 {
        return ((self.x.powi(2)) + (self.y.powi(2)) + (self.z.powi(2))).sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        return (self.x.powi(2)) + (self.y.powi(2)) + (self.z.powi(2));
    }

    pub fn make_unit_vector(&mut self) {
        let length = self.length();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
    }

    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();

        return *self / length;
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Self {
            x: self[1] * rhs[2] - self[2] * rhs[1],
            y: self[2] * rhs[0] - self[0] * rhs[2],
            z: self[0] * rhs[1] - self[1] * rhs[0],
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Index<i32> for Vec3 {
    type Output = f64;

    fn index(&self, i: i32) -> &f64 {
        return match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("invalid index: {}", i),
        };
    }
}

impl ops::IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, i: i32) -> &mut f64 {
        return match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("invalid index: {}", i),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vec3;

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(5.0, 7.0, 9.0)
        );
    }

    #[test]
    fn test_add_assign() {
        let mut first = Vec3::new(1.0, 2.0, 3.0);
        first += Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(first, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) - Vec3::new(4.0, 5.0, 2.0),
            Vec3::new(-3.0, -3.0, 1.0)
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut first = Vec3::new(1.0, 2.0, 3.0);
        first -= Vec3::new(4.0, 5.0, 2.0);

        assert_eq!(first, Vec3::new(-3.0, -3.0, 1.0));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_idx() {
        let first = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(first[2], 3.0);
    }

    #[test]
    fn test_idx_mut() {
        let mut first = Vec3::new(1.0, 2.0, 3.0);
        first[2] = -1.0;
        assert_eq!(first[2], -1.0);
    }

    #[test]
    fn test_cross() {
        let first = Vec3::new(-1.0, 7.0, 4.0);
        let second = Vec3::new(-5.0, 8.0, 4.0);
        let answer = Vec3::new(-4.0, -16.0, 27.0);
        assert_eq!(first.cross(second), answer);
    }

    #[test]
    fn test_unit() {
        let first = Vec3::new(-1.0, 7.0, 4.0);
        let eps = 0.000001_f64;
        assert!((1.0 - first.unit_vector().length()).abs() < eps);
    }
}
