use super::*;

pub struct Sphere {
    pub pos: Vector3f,
    pub radius: FloatingUnit,
    pub colour: u32,
}

impl Intersectable for Sphere {
    fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> Option<(u32, FloatingUnit)> {
        let direction = direction.normalize();
        let length = origin.subtract(&self.pos);
        let a = 1.0;
        let b = length.dot_product(&direction) * (2.0);
        let c = length.dot_product(&length) - self.radius;

        if let Some((mut x0, mut x1)) = solveQuadratic(a, b, c) {
            if x0 > x1 {
                std::mem::swap(&mut x0, &mut x1);
            }
            if x0.is_sign_negative() {
                if x1.is_sign_negative() {
                    return None;
                }
                return Some((self.colour, x1));
            }
            return Some((self.colour, x0));
        }
        return None;
    }
}

fn solveQuadratic(
    a: FloatingUnit,
    b: FloatingUnit,
    c: FloatingUnit,
) -> Option<(FloatingUnit, FloatingUnit)> {
    let discr: FloatingUnit = b * b - (4.0 * a * c);

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
