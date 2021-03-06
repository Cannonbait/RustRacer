use crate::model::color::*;
use crate::model::ray::*;
use crate::model::shape::*;
use crate::model::types::*;
use crate::model::vector::*;

pub struct Circle {
    pub pos: Vector3f,
    pub radius: Fu,
    pub color: Color,
}

impl Shape for Circle {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<Fu> {
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
            Some(distance)
        } else {
            None
        }
    }
    fn get_color(&self, ray: &Ray, t: Fu) -> Color {
        return self.color;
    }
}
