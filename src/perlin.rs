use crate::vec3::*;
use rand::Rng;

const point_count: usize = 256;
pub struct Perlin {
    rand_float: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let rand_float: Vec<f64> = (0..point_count)
            .map(|_| rand::thread_rng().gen::<f64>())
            .collect();

        Perlin {
            rand_float,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, point: Point) -> f64 {
        // let u = point.x - point.x.floor();
        // let v = point.y - point.y.floor();
        // let w = point.z - point.z.floor();

        let i = (4.0 * point.x) as usize & 255;
        let j = (4.0 * point.y) as usize & 255;
        let k = (4.0 * point.z) as usize & 255;

        self.rand_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut vec: Vec<usize> = (0..point_count).collect();

    for i in (1..point_count).rev() {
        vec.swap(i.into(), rand::thread_rng().gen_range(0, i).into());
    }

    return vec;
}
