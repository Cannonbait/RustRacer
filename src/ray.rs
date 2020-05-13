use super::*;
pub struct Ray {
    pub pos: Vector3f,
    pub dir: Vector3f,
}

impl Ray {
    pub fn intersect(&self, objects: &Vec<Box<dyn Intersectable>>) -> Option<u32> {
        for o in objects.iter() {
            if let Some(value) = o.intersects(&self.pos, &self.dir) {
                return Some(value);
            }
        }
        return None;
    }
}
