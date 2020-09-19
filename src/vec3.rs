use rand::*;
use std::f64::consts::*;
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn print(&self) {
        println!("{} {} {}", self.x, self.y, self.z);
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random(rng: &mut rngs::ThreadRng) -> Self {
        Self {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            z: rng.gen::<f64>(),
        }
    }

    pub fn random_range(rng: &mut rngs::ThreadRng, min: f64, max: f64) -> Self {
        Self {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    pub fn random_in_unit_sphere(rng: &mut rngs::ThreadRng) -> Self {
        loop {
            let vec = Self::random_range(rng, -1.0, 1.0);

            if vec.length() < 1.0 {
                return vec;
            }
        }
    }

    pub fn random_unit_vector(rng: &mut rngs::ThreadRng) -> Self {
        let a = rng.gen_range(0.0, TAU);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z: z,
        }
    }

    pub fn random_in_hemisphere(rng: &mut rngs::ThreadRng, normal: &Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }
}

impl Color {
    pub fn write_color(&self, samples_per_pixel: u8, string: &mut String) {
        let Color {
            x: mut r,
            y: mut g,
            z: mut b,
        } = self;

        let scale = 1.0 / samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let ir = (255.999 * r.clamp(0.0, 0.999)) as u8;
        let ig = (255.999 * g.clamp(0.0, 0.999)) as u8;
        let ib = (255.999 * b.clamp(0.0, 0.999)) as u8;

        string.push_str(&format!("{} {} {}\n", ir, ig, ib));
    }
}

use std::ops::*;

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<u8> for Vec3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Using index {} on a Vec3", index),
        }
    }
}

impl IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Using index {} on a Vec3", index),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
