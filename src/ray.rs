use super::*;
use std::cmp::Ordering;
pub struct Ray {
    pub pos: Vector3f,
    pub dir: Vector3f,
}

impl Ray {
    // Vec3f Phit = orig + dir * t;
    //     Vec3f Nhit;
    //     Vec2f tex;
    //     hitObject->getSurfaceData(Phit, Nhit, tex);
    //     // Use the normal and texture coordinates to shade the hit point.
    //     // The normal is used to compute a simple facing ratio and the texture coordinate
    //     // to compute a basic checker board pattern
    //     float scale = 4;
    //     float pattern = (fmodf(tex.x * scale, 1) > 0.5) ^ (fmodf(tex.y * scale, 1) > 0.5);
    //     hitColor = std::max(0.f, Nhit.dotProduct(-dir)) * mix(hitObject->color, hitObject->color * 0.8, pattern);
    // }

    fn get_color_for_ray(&self, shape: &Box<dyn Shape>, t: f64) -> Color {
        let (normalized_hit, tex) = shape.get_surface_data(&self.pos.add(&self.dir.multiply(t)));

        let scale = 8.0;
        let pattern = (((tex.x * scale) % 1.0) > 0.5) ^ (((tex.y * scale) % 1.0) > 0.5);
        let product = normalized_hit.dot_product(&self.dir.multiply(-1.0));

        let hit_color = if product > 0.0 {
            shape
                .get_color()
                .mix(
                    &shape.get_color().multiply(0.8),
                    if pattern { 1.0 } else { 0.0 },
                )
                .multiply(product)
        } else {
            Color { r: 0, g: 0, b: 0 }
        };

        hit_color
    }

    pub fn intersect(&self, shapes: &Vec<Box<dyn Shape>>) -> Option<Color> {
        shapes
            .iter()
            .filter_map(|shape| shape.intersects(&self.pos, &self.dir).map(|t| (shape, t)))
            .min_by(|(_, t0), (_, t1)| t0.partial_cmp(t1).unwrap_or(Ordering::Equal))
            .map(|(shape, t)| self.get_color_for_ray(shape, t))
    }
}
