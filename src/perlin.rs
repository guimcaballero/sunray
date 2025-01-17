use crate::vec3::*;
use rand::Rng;

const POINT_COUNT: usize = 256;
pub struct Perlin {
    rand_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let rand_vec: Vec<Vec3> = (0..POINT_COUNT)
            .map(|_| Vec3::random_in_unit_sphere())
            .collect();

        Perlin {
            rand_vec,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, point: Point) -> f32 {
        let ijk = Point {
            x: point.x.floor(),
            y: point.y.floor(),
            z: point.z.floor(),
        };
        let uvw = point - ijk;
        let uvw = uvw * uvw * (3.0 * Vec3::ones() - 2.0 * uvw);

        let mut corners = [[[Vec3::zeros(); 2]; 2]; 2];

        // Need to clean up this to use iterators
        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x_index = ((ijk.x as i16 + di as i16) & 255) as usize;
                    let y_index = ((ijk.y as i16 + dj as i16) & 255) as usize;
                    let z_index = ((ijk.z as i16 + dk as i16) & 255) as usize;

                    corners[di][dj][dk] = self.rand_vec[(self.perm_x[x_index]
                        ^ self.perm_y[y_index]
                        ^ self.perm_z[z_index])
                        as usize];
                }
            }
        }

        trilinear_interpolation(&corners, uvw)
    }

    pub fn turbulence(&self, point: Point, depth: u8) -> f32 {
        let mut accum = 0.0;
        let mut weight = 1.0;
        let mut temp_p = point;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 4.0;
        }

        accum.abs()
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut vec: Vec<usize> = (0..POINT_COUNT).collect();

    let mut rng = rand::thread_rng();
    for i in (1..POINT_COUNT).rev() {
        vec.swap(i, rng.gen_range(0, i));
    }

    vec
}

type Corners = [[[Vec3; 2]; 2]; 2];
fn trilinear_interpolation(corners: &Corners, uvw: Vec3) -> f32 {
    let mut accum = 0.0;

    let one_minus_uvw = Vec3::ones() - uvw;

    for (i, corner2) in corners.iter().enumerate() {
        for (j, corner1) in corner2.iter().enumerate() {
            for (k, corner) in corner1.iter().enumerate() {
                let ijk = Vec3::new(i as f32, j as f32, k as f32);
                let one_minus_ijk = Vec3::ones() - ijk;

                accum += (ijk * uvw + one_minus_ijk * one_minus_uvw).multiply_components()
                    * (uvw - ijk).dot(&corner);
            }
        }
    }

    accum
}
