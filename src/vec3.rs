use std::ops;

struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        return self.e[0];
    }
    pub fn y(&self) -> f64 {
        return self.e[1];
    }
    pub fn z(&self) -> f64 {
        return self.e[2];
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}
impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}
