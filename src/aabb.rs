use crate::{ray::*, vec3::*};

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Point,
    pub max: Point,
}

impl AABB {
    // Returns the minimum t of the ray that is inside the AABB
    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<f32> {
        // TODO What happens if the ray starts inside the bounding box?
        let mut min = tmin;
        for a in 0..3 {
            let t0 = ((self.min[a] - ray.origin[a]) / ray.direction[a])
                .min((self.max[a] - ray.origin[a]) / ray.direction[a]);
            let t1 = ((self.min[a] - ray.origin[a]) / ray.direction[a])
                .max((self.max[a] - ray.origin[a]) / ray.direction[a]);

            min = t0.max(min);
            let tmax = t1.min(tmax);

            if tmax <= min {
                return None;
            }
        }

        // Check that we actually got a better t
        assert!(min >= tmin);

        Some(min)
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
