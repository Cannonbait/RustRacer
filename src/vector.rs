use super::*;
#[derive(Debug, Copy, Clone)]
pub struct Vector3f {
    pub x: Fu,
    pub y: Fu,
    pub z: Fu,
}

impl Vector3f {
    pub fn new(x: u32, y: u32, z: u32) -> Vector3f {
        Vector3f {
            x: x as Fu,
            y: y as Fu,
            z: z as Fu,
        }
    }

    pub fn distance(&self, other: &Vector3f) -> Fu {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn magnitude(&self) -> Fu {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn dot_product(&self, other: &Vector3f) -> Fu {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn multiply(&self, operand: Fu) -> Vector3f {
        Vector3f {
            x: self.x * operand,
            y: self.y * operand,
            z: self.z * operand,
        }
    }

    pub fn add(&self, other: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract(&self, other: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn normalize(&self) -> Vector3f {
        let length = self.magnitude();

        Vector3f {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}
