use super::*;
pub struct Plane {
    pub pos: Vector3f,
    pub normal: Vector3f,
    pub color: Color,
}

impl Plane {
    pub fn new(pos: Vector3f, normal: Vector3f, color: Color) -> Plane {
        Plane {
            pos,
            normal: normal.normalize(),
            color,
        }
    }
}

impl Shape for Plane {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<Fu> {
        let denom = self.normal.dot_product(&direction);
        if denom.abs() > 0.00000001 {
            let dist = self.pos.subtract(origin);
            let t = dist.dot_product(&self.normal) / denom;
            if t >= 0.0 {
                return Some(t);
            }
        }
        return None;
    }

    fn get_color(&self, ray: &Ray, t: Fu) -> Color {
        return self.color;
    }
}
