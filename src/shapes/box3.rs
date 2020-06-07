use std::mem;

use crate::model::color::*;
use crate::model::ray::*;
use crate::model::shape::*;
use crate::model::types::*;
use crate::model::vector::*;

pub struct Box3 {
    pub min: Vector3f,
    pub max: Vector3f,
    pub color: Color,
}

impl Shape for Box3 {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<Fu> {
        let mut tmin = (self.min.x - origin.x) / direction.x;
        let mut tmax = (self.max.x - origin.x) / direction.x;

        if tmin > tmax {
            mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min.y - origin.y) / direction.y;
        let mut tymax = (self.max.y - origin.y) / direction.y;

        if tymin > tymax {
            mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return None;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }
        let mut tzmin = (self.min.z - origin.z) / direction.z;
        let mut tzmax = (self.max.z - origin.z) / direction.z;

        if tzmin > tzmax {
            mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }
        return Some(tmin);
    }
    fn get_color(&self, ray: &Ray, t: Fu) -> Color {
        return self
            .color
            .multiply(self.get_normal(&ray, t).dot_product(&ray.dir).abs());
    }
}

impl Box3 {
    fn get_normal(&self, ray: &Ray, t: Fu) -> Vector3f {
        let hit = &ray.pos.add(&ray.dir.multiply(t));
        let epsilon = 0.000000001;
        if (hit.x - self.min.x).abs() < epsilon {
            Vector3f {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            }
        } else if (hit.x - self.max.x).abs() < epsilon {
            Vector3f {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        } else if (hit.z - self.min.z).abs() < epsilon {
            Vector3f {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }
        } else if (hit.z - self.max.z).abs() < epsilon {
            Vector3f {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        } else if (hit.y - self.min.y).abs() < epsilon {
            Vector3f {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            }
        } else if (hit.y - self.max.y).abs() < epsilon {
            Vector3f {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
        } else {
            Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }
}
