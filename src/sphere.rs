use super::*;
use std::f64::consts;

pub struct Sphere {
    pub pos: Vector3f,
    pub radius: Fu,
    pub color: u32,
}

impl Shape for Sphere {
    fn get_surface_data(&self, hit: &Vector3f) -> (Vector3f, Vector3f) {
        let normalized_hit = hit.subtract(&self.pos).normalize();
        let texture_coord = Vector3f {
            x: 1.0 + (normalized_hit.z.atan2(normalized_hit.x) / consts::PI) * 0.5,
            y: normalized_hit.z.acos() / consts::PI,
            z: 0.0,
        };

        (normalized_hit, texture_coord)
    }

    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<Fu> {
        let direction = direction.normalize();
        let length = origin.subtract(&self.pos);
        let a = 1.0;
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
    fn get_color(&self) -> Cu {
        self.color
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
