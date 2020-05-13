use super::*;
use std::cmp::Ordering;
pub struct Ray {
    pub pos: Vector3f,
    pub dir: Vector3f,
}

impl Ray {
    pub fn intersect(&self, objects: &Vec<Box<dyn Intersectable>>) -> Option<u32> {
        objects
            .iter()
            .filter_map(|o| o.intersects(&self.pos, &self.dir))
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(color, _)| color)
    }
}
