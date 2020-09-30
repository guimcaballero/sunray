use crate::{ray::*, vec3::*};

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Point,
    pub max: Point,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let t0 = ((self.min[a] - ray.origin[a]) / ray.direction[a])
                .min((self.max[a] - ray.origin[a]) / ray.direction[a]);
            let t1 = ((self.min[a] - ray.origin[a]) / ray.direction[a])
                .max((self.max[a] - ray.origin[a]) / ray.direction[a]);

            let tmin = t0.max(tmin);
            let tmax = t1.min(tmax);

            if tmax <= tmin {
                return false;
            }
        }

        return true;
    }

    pub fn surrounding_box(&self, other: AABB) -> AABB {
        let min = Point {
            x: self.min.x.min(other.min.x),
            y: self.min.y.min(other.min.y),
            z: self.min.z.min(other.min.z),
        };
        let max = Point {
            x: self.max.x.max(other.max.x),
            y: self.max.y.max(other.max.y),
            z: self.max.z.max(other.max.z),
        };

        AABB { min, max }
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: Point::zeros(),
            max: Point::zeros(),
        }
    }
}
