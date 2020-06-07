use super::color::*;
use super::ray::*;
use super::types::*;
use super::vector::*;

pub trait Shape {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<Fu>;
    fn get_color(&self, ray: &Ray, t: Fu) -> Color;
}
