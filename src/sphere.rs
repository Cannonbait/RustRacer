use super::*;
use std::f64::consts;

pub struct Sphere {
    pub pos: Vector3f,
    pub radius: Fu,
    pub color: Color,
}

impl Shape for Sphere {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<Fu> {
        let direction = direction.normalize();
        let length = origin.subtract(&self.pos);
        let a = direction.dot_product(&direction);
        let b = length.dot_product(&direction) * 2.0;
        let c = length.dot_product(&length) - self.radius * self.radius;

        if let Some((mut x0, mut x1)) = solve_quadratic(a, b, c) {
            if x0 > x1 {
                std::mem::swap(&mut x0, &mut x1);
            }
            if x0.is_sign_negative() {
                if x1.is_sign_negative() {
                    return None;
                }
                return Some(x1);
            }
            return Some(x0);
        }
        return None;
    }

    fn get_color(&self, ray: &Ray, t: Fu) -> Color {
        let (normalized_hit, tex) = self.get_surface_data(&ray.pos.add(&ray.dir.multiply(t)));

        let scale = 8.0;
        let pattern = (((tex.x * scale) % 1.0) > 0.5) ^ (((tex.y * scale) % 1.0) > 0.5);
        let product = normalized_hit.dot_product(&ray.dir.multiply(-1.0));

        if product < 0.0 {
            return Color { r: 0, g: 0, b: 0 };
        }
        return self
            .color
            .mix(&self.color.multiply(0.8), if pattern { 1.0 } else { 0.0 })
            .multiply(product);
    }
}

impl Sphere {
    fn get_surface_data(&self, hit: &Vector3f) -> (Vector3f, Vector3f) {
        let normalized_hit = hit.subtract(&self.pos).normalize();
        let texture_coord = Vector3f {
            x: (1.0 + (normalized_hit.z.atan2(normalized_hit.x) / consts::PI)) * 0.5,
            y: normalized_hit.y.acos() / consts::PI,
            z: 0.0,
        };

        (normalized_hit, texture_coord)
    }
}
fn solve_quadratic(a: Fu, b: Fu, c: Fu) -> Option<(Fu, Fu)> {
    let discr: Fu = b * b - (4.0 * a * c);

    if discr.is_sign_negative() {
        return None;
    } else if discr == 0.0 {
        return Some((-0.5 * b / a, -0.5 * b / a));
    }
    let q = if b.is_sign_positive() {
        -0.5 * (b + discr.sqrt())
    } else {
        -0.5 * (b - discr.sqrt())
    };
    return Some((q / a, c / q));
}
