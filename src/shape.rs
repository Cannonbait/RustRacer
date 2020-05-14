use super::*;
pub struct Circle {
    pub pos: Vector3f,
    pub radius: FloatingUnit,
    pub colour: u32,
}
pub trait Intersectable {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<(u32, FloatingUnit)>;
}

impl Intersectable for Circle {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<(u32, FloatingUnit)> {
        let distance = origin.subtract(&self.pos).magnitude();
        let multiple = distance / direction.z;
        // println!("Circle distance {}", distance);

        let hit_x = origin.x + direction.x * multiple;
        let hit_y = origin.y + direction.y * multiple;

        let hit = self.pos.distance(&Vector3f {
            x: hit_x,
            y: hit_y,
            z: self.pos.z,
        }) < self.radius;

        if hit {
            Some((self.colour, distance))
        } else {
            None
        }
    }
}
