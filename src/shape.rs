use super::*;

pub trait Shape {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<Fu>;
    fn get_color(&self, ray: &Ray, t: Fu) -> Color;
}
