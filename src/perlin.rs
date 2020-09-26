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
        let ijk = Point {
            x: point.x.floor(),
            y: point.y.floor(),
            z: point.z.floor(),
        };
        let uvw = point - ijk;
        let uvw = uvw * uvw * (3.0 * Vec3::ones() - 2.0 * uvw);

        let mut corners = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x_index = ((ijk.x as i16 + di as i16) & 255) as usize;
                    let y_index = ((ijk.y as i16 + dj as i16) & 255) as usize;
                    let z_index = ((ijk.z as i16 + dk as i16) & 255) as usize;
                    corners[di][dj][dk] = self.rand_float[(self.perm_x[x_index]
                        ^ self.perm_y[y_index]
                        ^ self.perm_z[z_index])
                        as usize];
                }
            }
        }

        trilinear_interpolation(&corners, uvw)
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut vec: Vec<usize> = (0..point_count).collect();

    for i in (1..point_count).rev() {
        vec.swap(i.into(), rand::thread_rng().gen_range(0, i).into());
    }

    return vec;
}

type Corners = [[[f64; 2]; 2]; 2];
fn trilinear_interpolation(corners: &Corners, uvw: Vec3) -> f64 {
    let mut accum = 0.0;

    let one_minus_uvw = Vec3::ones() - uvw;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let ijk = Vec3::new(i as f64, j as f64, k as f64);
                let one_minus_ijk = Vec3::ones() - ijk;

                accum += (ijk * uvw + one_minus_ijk * one_minus_uvw).multiply_components()
                    * corners[i][j][k];
            }
        }
    }

    accum
}
