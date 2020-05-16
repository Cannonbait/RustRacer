use super::*;
use std::cmp::Ordering;
pub struct Ray {
    pub pos: Vector3f,
    pub dir: Vector3f,
}

impl Ray {
    pub fn new(pos: Vector3f, dir: Vector3f) -> Ray {
        Ray {
            pos,
            dir: dir.normalize(),
        }
    }

    pub fn intersect(&self, shapes: &Vec<Box<dyn Shape>>) -> Option<Color> {
        shapes
            .iter()
            .filter_map(|shape| shape.intersects(&self.pos, &self.dir).map(|t| (shape, t)))
            .min_by(|(_, t0), (_, t1)| t0.partial_cmp(t1).unwrap_or(Ordering::Equal))
            .map(|(shape, t)| shape.get_color(&self, t))
    }
}
