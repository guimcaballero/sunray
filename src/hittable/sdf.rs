use crate::{hittable::*, material::*};
use std::f32::consts::*;

pub struct TracedSDF {
    pub sdf: Box<dyn SDF>,
    pub material: Material,
}

impl TracedSDF {
    pub fn normal(&self, point: Vec3) -> Vec3 {
        let h = 0.0001;

        let xyy = Vec3::new(1., -1., -1.);
        let yyx = Vec3::new(-1., -1., 1.);
        let yxy = Vec3::new(-1., 1., -1.);
        let xxx = Vec3::ones();

        (xyy * self.sdf.dist(point + xyy * h)
            + yyx * self.sdf.dist(point + yyx * h)
            + yxy * self.sdf.dist(point + yxy * h)
            + xxx * self.sdf.dist(point + xxx * h))
        .normalize()
    }
}

impl Hittable for TracedSDF {
    fn hit(&self, ray: &Ray, taemin: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        // Start from a t in the bounding box, and not from taemin
        let bounding_box_tmin = self
            .bounding_box(0., 0.)
            .and_then(|aabb| aabb.hit(ray, taemin, t_max));

        let mut t = if let Some(tmin) = bounding_box_tmin {
            tmin
        } else {
            taemin
        };

        for _ in 0..2000 {
            let point = ray.at(t);
            let distance = self.sdf.dist(point);

            if distance < 0.00001 {
                let normal = self.normal(point);
                *hit_record = HitRecord {
                    point,
                    normal,
                    t,
                    material: self.material.clone(),
                    ..*hit_record
                };
                hit_record.set_face_normal(&ray, &normal);

                return true;
            }
            if distance > 10000.0 || t > t_max {
                break;
            }

            t += distance;
        }

        false
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.sdf.bounding_box(t0, t1)
    }
}

pub trait SDF: Send + Sync {
    fn dist(&self, position: Vec3) -> f32;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

pub struct SDFSphere {
    pub radius: f32,
    pub center: Vec3,
}

impl SDF for SDFSphere {
    fn dist(&self, position: Vec3) -> f32 {
        (position - self.center).length() - self.radius
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Point::from(self.radius),
            max: self.center + Point::from(self.radius),
        })
    }
}

pub struct SDFCilinder {
    pub radius: f32,
    pub center: Vec3,
}

impl SDF for SDFCilinder {
    fn dist(&self, position: Vec3) -> f32 {
        Vec3::new(position.x - self.center.x, position.z - self.center.z, 0.).length() - self.radius
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        None
    }
}

pub struct SDFPlane {
    pub normal: Vec3,
    pub h: f32,
}

impl SDF for SDFPlane {
    fn dist(&self, position: Vec3) -> f32 {
        position.dot(&self.normal) + self.h
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        None
    }
}

pub struct SDFDonut {
    pub radius0: f32,
    pub radius1: f32,
    pub center: Vec3,
}

impl SDF for SDFDonut {
    fn dist(&self, position: Vec3) -> f32 {
        let qx = Vec3::new(position.y - self.center.y, position.x - self.center.x, 0.0).length()
            - self.radius0;
        let qy = position.z - self.center.z;

        Vec3::new(qx, qy, 0.0).length() - self.radius1
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Point::new(
                -self.radius0 - self.radius1,
                -self.radius1,
                -self.radius0 - self.radius1,
            ),
            max: Point::new(
                self.radius0 + self.radius1,
                self.radius1,
                self.radius0 + self.radius1,
            ),
        })
    }
}

pub struct SDFCube {
    pub dimensions: Vec3,
    pub center: Point,
}

impl SDF for SDFCube {
    fn dist(&self, position: Vec3) -> f32 {
        let q = (position - self.center).abs() - self.dimensions;
        q.max(0.0).length() + q.x.max(q.y.max(q.z)).min(0.0)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - self.dimensions,
            max: self.center + self.dimensions,
        })
    }
}

pub struct SDFOctahedron {
    pub size: f32,
    pub center: Point,
}

impl SDF for SDFOctahedron {
    fn dist(&self, position: Vec3) -> f32 {
        let p = (position - self.center).abs();
        (p.x + p.y + p.z - self.size) * 0.577_350_26
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::from(self.size),
            max: self.center + Vec3::from(self.size),
        })
    }
}

pub struct SDFMandelBulb {
    pub center: Point,
}

impl SDF for SDFMandelBulb {
    fn dist(&self, position: Vec3) -> f32 {
        let mut w = position - self.center;
        let mut m = w.length_squared(); // 300

        let mut dz = 1.0;

        for _ in 0..15 {
            dz = 8. * m.sqrt().powf(7.0) * dz + 1.0;
            let r = w.length();
            let b = 8. * (w.y / r).acos();
            let a = 8. * w.x.atan2(w.z);
            w = position
                + r.powf(8.) * Vec3::new((b).sin() * (a).sin(), (b).cos(), (b).sin() * (a).cos());

            m = w.length_squared();
            if m > 256.0 {
                break;
            }
        }
        return 0.25 * m.ln() * m.sqrt() / dz;
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::from(2.),
            max: self.center + Vec3::from(2.),
        })
    }
}

pub struct SDFMandelBox {
    pub center: Point,
    pub scale: f32,
}
impl SDF for SDFMandelBox {
    fn dist(&self, mut z: Vec3) -> f32 {
        let offset = z - self.center;
        let mut dr = 1.0;
        for _ in 0..20 {
            z = box_fold(z, 1.); // Reflect
            let (z2, dr2) = sphere_fold(z, dr, 0.1, 1.5); // Sphere Inversion
            z = z2;
            dr = dr2;

            z = self.scale * z + offset; // Scale & Translate
            dr = dr * self.scale.abs() + 1.0;
        }
        let r = z.length();
        r / dr.abs()
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::from(10.),
            max: self.center + Vec3::from(10.),
        })
    }
}

fn sphere_fold(mut z: Vec3, mut dz: f32, min_rad: f32, fixed_rad: f32) -> (Vec3, f32) {
    let r2 = z.length_squared();
    if r2 < min_rad {
        // linear inner scaling
        let temp = fixed_rad / min_rad;
        z = z * temp;
        dz *= temp;
    } else if r2 < fixed_rad {
        // this is the actual sphere inversion
        let temp = fixed_rad / r2;
        z = z * temp;
        dz *= temp;
    }

    (z, dz)
}

fn box_fold(z: Vec3, folding_limit: f32) -> Vec3 {
    z.clamp(-folding_limit, folding_limit) * 2.0 - z
}

pub struct SDFKnot {
    pub center: Point,
    pub k: f32,
}
impl SDF for SDFKnot {
    fn dist(&self, position: Vec3) -> f32 {
        let mut p = position - self.center;

        let r = Vec3::new(p.x, p.y, 0.).length();
        let mut a = p.y.atan2(p.x);
        let oa = self.k * a;

        a = a.rem_euclid(0.001 * TAU) - 0.001 * TAU / 2.;

        p.x = r * a.cos();
        p.y = r * a.sin();
        p.x -= 6.0;

        let old_px = p.x;
        p.x = oa.cos() * p.x - oa.sin() * p.z;
        p.z = oa.cos() * p.z + oa.sin() * old_px;

        p.x = p.x.abs() - 1.35;
        p.length() - 1.
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::from(100.),
            max: self.center + Vec3::from(100.),
        })
    }
}

pub struct SDFUnion {
    pub a: Box<dyn SDF>,
    pub b: Box<dyn SDF>,
}

impl SDF for SDFUnion {
    fn dist(&self, position: Vec3) -> f32 {
        self.a.dist(position).min(self.b.dist(position))
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(box_a) = self.a.bounding_box(t0, t1) {
            if let Some(box_b) = self.b.bounding_box(t0, t1) {
                return Some(box_a.surrounding_box(box_b));
            }
        }
        None
    }
}

pub struct SDFSubstraction {
    pub a: Box<dyn SDF>,
    pub b: Box<dyn SDF>,
}

impl SDF for SDFSubstraction {
    fn dist(&self, position: Vec3) -> f32 {
        self.a.dist(position).max(-self.b.dist(position))
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.a.bounding_box(t0, t1)
    }
}

pub struct SDFIntersection {
    pub a: Box<dyn SDF>,
    pub b: Box<dyn SDF>,
}

impl SDF for SDFIntersection {
    fn dist(&self, position: Vec3) -> f32 {
        self.a.dist(position).max(self.b.dist(position))
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        // We just have to return any of the two boxes, as the intersection is inside
        if let Some(box_a) = self.a.bounding_box(t0, t1) {
            return Some(box_a);
        }
        if let Some(box_b) = self.b.bounding_box(t0, t1) {
            return Some(box_b);
        }
        None
    }
}

pub struct SDFRepetition {
    pub a: Box<dyn SDF>,
    pub repetition: Vec3,
}

impl SDF for SDFRepetition {
    fn dist(&self, position: Vec3) -> f32 {
        let q = (position + 0.5 * self.repetition).modulo(self.repetition) - 0.5 * self.repetition;
        self.a.dist(q)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        None
    }
}

pub struct SDFScale {
    pub a: Box<dyn SDF>,
    pub scale: f32,
}

impl SDF for SDFScale {
    fn dist(&self, position: Vec3) -> f32 {
        self.a.dist(position / self.scale) * self.scale
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(aabb) = self.a.bounding_box(t0, t1) {
            let center = aabb.center();
            let min = self.scale * (aabb.min - center);
            let max = self.scale * (aabb.max - center);
            Some(AABB {
                min: min + center,
                max: max + center,
            })
        } else {
            None
        }
    }
}

pub struct SDFDebugBounding {
    pub a: Box<dyn SDF>,
    pub debug: bool,
}

impl SDF for SDFDebugBounding {
    fn dist(&self, position: Vec3) -> f32 {
        if !self.debug {
            return self.a.dist(position);
        }

        if let Some(aabb) = self.a.bounding_box(0., 0.) {
            let center = aabb.center();
            let max = aabb.max - center;

            let q = (position - center).abs() - max;
            q.max(0.0).length() + q.x.max(q.y.max(q.z)).min(0.0)
        } else {
            f32::MAX
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.a.bounding_box(t0, t1)
    }
}

pub fn cross(center: Point, dimension: f32) -> Box<dyn SDF> {
    let a = SDFUnion {
        a: Box::new(SDFCube {
            center,
            dimensions: Vec3::new(f32::INFINITY, 1., 1.) * dimension,
        }),
        b: Box::new(SDFCube {
            center,
            dimensions: Vec3::new(1., f32::INFINITY, 1.) * dimension,
        }),
    };
    let b = SDFCube {
        center,
        dimensions: Vec3::new(1., 1., f32::INFINITY) * dimension,
    };

    Box::new(SDFUnion {
        a: Box::new(a),
        b: Box::new(b),
    })
}

pub fn menger_sponge(iterations: usize) -> Box<TracedSDF> {
    let size = 33.;
    let a = Box::new(SDFCube {
        center: Point::zeros(),
        dimensions: Vec3::from(size),
    });

    let mut b = sdf::cross(Point::zeros(), (1. / 3.) * size);
    for i in 1..=iterations {
        b = Box::new(SDFUnion {
            a: Box::new(SDFRepetition {
                a: sdf::cross(Point::zeros(), size / f32::powi(3., i as i32)),
                repetition: Vec3::from(size * 2. / f32::powi(3., (i as i32) - 1)),
            }),
            b,
        });
    }

    box TracedSDF {
        sdf: Box::new(SDFSubstraction { a, b }),
        material: Material::Lambertian(Color::new(0.8, 0.1, 0.1)),
    }
}

#[allow(dead_code)]
pub fn finite_plane(center: Point, dimension: f32) -> Box<dyn SDF> {
    let a = SDFPlane {
        normal: Vec3::new(0., 1., 0.),
        h: 0.,
    };
    let b = SDFCube {
        center,
        dimensions: Vec3::from(dimension),
    };

    Box::new(SDFIntersection {
        a: Box::new(a),
        b: Box::new(b),
    })
}
