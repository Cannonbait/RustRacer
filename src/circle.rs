use super::*;
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
    fn get_surface_data(&self, hit: &Vector3f) -> (Vector3f, Vector3f) {
        todo!()
    }
    fn get_color(&self) -> Color {
        self.color
    }
}
