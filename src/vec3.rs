use core::panic;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    #[allow(dead_code)]
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    #[allow(dead_code)]
    pub fn x(&self) -> f64 {
        return self.e[0];
    }
    #[allow(dead_code)]
    pub fn y(&self) -> f64 {
        return self.e[1];
    }
    #[allow(dead_code)]
    pub fn z(&self) -> f64 {
        return self.e[2];
    }

    #[allow(dead_code)]
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    #[allow(dead_code)]
    pub fn length(&self) -> f64 {
        let tmp: f64 = self.length_squared().sqrt();
        return tmp;
    }

    #[allow(dead_code)]
    pub fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }

    #[allow(dead_code)]
    pub fn div_assign(&mut self, t: f64) {
        if t == 0.0 {
            panic!("division by zero is nono");
        }
        self.e[0] /= t;
        self.e[1] /= t;
        self.e[2] /= t;
    }

    #[allow(dead_code)]
    pub fn div(self, t: f64) -> Self {
        if t == 0.0 {
            panic!("division by zero is nono");
        }
        Self {
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
        }
    }

    #[allow(dead_code)]
    pub fn mul(&self, scalar: f64) -> Self {
        Self {
            e: [self.e[0] * scalar, self.e[1] * scalar, self.e[2] * scalar],
        }
    }

    #[allow(dead_code)]
    pub fn dot(&self, other: &Self) -> f64 {
        return self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2];
    }

    #[allow(dead_code)]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }

    #[allow(dead_code)]
    pub fn unit_vector(&self) -> Self {
        return *self / self.length();
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
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

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, v: Self) {
        self.e[0] = self.e[0] + v.e[0];
        self.e[1] = self.e[1] + v.e[1];
        self.e[2] = self.e[2] + v.e[2];
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

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            e: [self.e[0] * scalar, self.e[1] * scalar, self.e[2] * scalar],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self {
        if t == 0.0 {
            panic!("division by zero is nono");
        }
        Self {
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}

type Color = Vec3;

pub fn write_color(file: &mut File, pixel_color: &Color) -> Result<(), std::io::Error> {
    let r = pixel_color.e[0];
    let g = pixel_color.e[1];
    let b = pixel_color.e[2];

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    // Write out the pixel color components.
    writeln!(file, "{} {} {}", rbyte, gbyte, bbyte)?;

    Ok(())
}

pub fn write_color_png(
    file: &mut image::RgbImage,
    pixel_color: &Color,
    x: u32,
    y: u32,
) -> Result<(), std::io::Error> {
    let r = pixel_color.e[0];
    let g = pixel_color.e[1];
    let b = pixel_color.e[2];

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    // Write out the pixel color components.
    file.put_pixel(x, y, image::Rgb([rbyte as u8, gbyte as u8, bbyte as u8]));
    // writeln!(file, "{} {} {}", rbyte, gbyte, bbyte)?;

    Ok(())
}



