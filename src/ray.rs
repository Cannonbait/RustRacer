use super::*;
use std::cmp::Ordering;
pub struct Ray {
    pub pos: Vector3f,
    pub dir: Vector3f,
}

impl Ray {
    pub fn intersect(&self, shapes: &Vec<Box<dyn Shape>>, default_color: Cu) -> Cu {
        shapes
            .iter()
            .filter_map(|shape| shape.intersects(&self.pos, &self.dir).map(|t| (shape, t)))
            .min_by(|(_, t0), (_, t1)| t0.partial_cmp(t1).unwrap_or(Ordering::Equal))
            .map_or(default_color, |(shape, t)| {
                let (normalized_hit, tex) = shape.get_surface_data(&self.dir.multiply(t));

                let scale = 4.0;
                let pattern = (((tex.x * scale) % 1.0) > 0.5) ^ (((tex.y * scale) % 1.0) > 0.5);
                let product = normalized_hit.dot_product(&self.dir.multiply(-1.0));

                let hit_color = (if product > 0.0 { product } else { 0.0 }
                    * mix(
                        shape.get_color(),
                        ((shape.get_color() as Fu) * 0.8) as Cu,
                        if pattern { 1.0 } else { 0.0 },
                    ) as Fu) as Cu;

                hit_color
            })
    }
}

fn mix(a: Cu, b: Cu, mix_value: Fu) -> Cu {
    return ((a as Fu) * ((1 as Fu) - mix_value) + (b as Fu) * mix_value) as Cu;
}
