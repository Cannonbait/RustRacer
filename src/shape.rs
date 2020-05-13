use super::*;
pub struct Rectangle {
    pub pos: Vector3f,
    pub width: f32,
    pub height: f32,
    pub colour: u32,
}
pub struct Circle {
    pub pos: Vector3f,
    pub radius: f32,
    pub colour: u32,
}
pub trait Intersectable {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<u32>;
}

impl Intersectable for Rectangle {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<u32> {
        let distance = self.pos.z - origin.z;
        let multiple = distance / direction.z;

        let hit_x = direction.x * multiple;
        let hit_y = direction.y * multiple;

        let hit = self.pos.x < hit_x
            && self.pos.x + self.width > hit_x
            && self.pos.y < hit_y
            && self.pos.y + self.height > hit_y;

        if hit {
            Some(self.colour)
        } else {
            None
        }
    }
}

impl Intersectable for Circle {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<u32> {
        let distance = self.pos.z - origin.z;
        let multiple = distance / direction.z;

        let hit_x = direction.x * multiple;
        let hit_y = direction.y * multiple;

        let hit = self.pos.distance(&Vector3f {
            x: hit_x,
            y: hit_y,
            z: self.pos.z,
        }) < self.radius;

        if hit {
            Some(self.colour)
        } else {
            None
        }
    }
}
