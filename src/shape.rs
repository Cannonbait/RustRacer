use super::*;

pub trait Shape {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<Fu>;
    fn get_surface_data(&self, hit: &Vector3f) -> (Vector3f, Vector3f);
    fn get_color(&self) -> Color;
}
