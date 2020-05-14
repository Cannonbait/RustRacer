use super::*;
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_u32(&self) -> u32 {
        let (r, g, b) = (self.r as u32, self.g as u32, self.b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn multiply(&self, operand: Fu) -> Color {
        Color {
            r: ((self.r as Fu) * operand) as u8,
            g: ((self.g as Fu) * operand) as u8,
            b: ((self.b as Fu) * operand) as u8,
        }
    }

    pub fn mix(&self, other: &Color, mix_value: Fu) -> Color {
        let self_multiplier = 1.0 - mix_value;
        let other_multiplier = mix_value;
        Color {
            r: ((self.r as Fu) * self_multiplier + (other.r as Fu) * other_multiplier) as u8,
            g: ((self.g as Fu) * self_multiplier + (other.g as Fu) * other_multiplier) as u8,
            b: ((self.b as Fu) * self_multiplier + (other.b as Fu) * other_multiplier) as u8,
        }
    }
}
