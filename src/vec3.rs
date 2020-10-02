use rand::*;
use std::f32::consts::*;
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    #[inline(always)]
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn print(&self) {
        println!("{} {} {}", self.x, self.y, self.z);
    }

    #[inline(always)]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline(always)]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    #[inline(always)]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
    #[inline(always)]
    pub fn max(&self, val: f32) -> Self {
        Self {
            x: self.x.max(val),
            y: self.y.max(val),
            z: self.z.max(val),
        }
    }
    #[inline(always)]
    pub fn min(&self, val: f32) -> Self {
        Self {
            x: self.x.min(val),
            y: self.y.min(val),
            z: self.z.min(val),
        }
    }
    #[inline(always)]
    pub fn modulo(&self, other: Vec3) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
        }
    }

    #[inline(always)]
    pub fn multiply_components(&self) -> f32 {
        self.x * self.y * self.z
    }

    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn from(val: f32) -> Self {
        Self::new(val, val, val)
    }

    #[inline(always)]
    pub fn zeros() -> Self {
        Self::from(0.0)
    }

    #[inline(always)]
    pub fn ones() -> Self {
        Self::from(1.0)
    }

    #[inline(always)]
    pub fn infinity() -> Self {
        Self::from(f32::INFINITY)
    }

    #[inline(always)]
    pub fn neg_infinity() -> Self {
        Self::from(-f32::INFINITY)
    }

    #[inline(always)]
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            z: rng.gen::<f32>(),
        }
    }

    #[inline(always)]
    pub fn random_range(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    #[inline(always)]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let vec = Self::random_range(-1.0, 1.0);

            if vec.length() < 1.0 {
                return vec;
            }
        }
    }

    #[inline(always)]
    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0.0, TAU);
        let z: f32 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z: z,
        }
    }

    #[inline(always)]
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }

    #[inline(always)]
    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let vec = Self {
                x: rng.gen_range(-1.0, 1.0),
                y: rng.gen_range(-1.0, 1.0),
                z: 0.0,
            };

            if vec.length_squared() < 1.0 {
                return vec;
            }
        }
    }

    #[inline(always)]
    pub fn reflect(&self, other: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(other) * *other
    }

    #[inline(always)]
    pub fn refract(&self, normal: &Vec3, eta_over_etai: f32) -> Self {
        let cos_theta = self.dot(&-*normal);
        let r_out_perp = eta_over_etai * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }
}

impl Color {
    pub fn write_color(&self, samples_per_pixel: u16) -> String {
        let Color {
            x: mut r,
            y: mut g,
            z: mut b,
        } = self;

        let scale = 1.0 / samples_per_pixel as f32;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let ir = (255.999 * r.clamp(0.0, 0.999)) as u16;
        let ig = (255.999 * g.clamp(0.0, 0.999)) as u16;
        let ib = (255.999 * b.clamp(0.0, 0.999)) as u16;

        format!("{} {} {}\n", ir, ig, ib)
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

impl Index<u16> for Vec3 {
    type Output = f32;

    fn index(&self, index: u16) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Using index {} on a Vec3", index),
        }
    }
}

impl IndexMut<u16> for Vec3 {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
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

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f32 {
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

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
