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
            .min_by(|(_, d0), (_, d1)| d0.partial_cmp(d1).unwrap_or(Ordering::Equal))
            .map(|(color, _)| color)
    }
}
