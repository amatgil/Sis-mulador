use std::ops::{Add, Shl, Sub};

use crate::Reg;

impl Add for Reg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Reg {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Reg {
    pub fn sha(self, rhs: Self) -> Self {
        let lsbs = rhs.0 & 0b01111;
        let sign = rhs.0 & 0b10000;

        let bit_to_extend = self.0 & (1 << 15) > 0;
        let mask = (-1 << (15 - lsbs)) * bit_to_extend as i16;

        println!("SHA: {}, {}, bit: {}. Mask is {}", self.0, rhs.0, bit_to_extend, mask);

        let n = if sign > 0 { // rhs is negative, we shift right
            self.0 >> lsbs
        } else { // rhs is positive, we shift left
            self.0 << lsbs
        };

        Reg(n | mask)
    }
}
impl Shl for Reg {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        let lsbs = rhs.0 & 0b01111;
        let sign = rhs.0 & 0b10000;
        
        let n = if sign > 0 { // rhs is negative, we shift right
            self.0 >> lsbs
        } else { // rhs is positive, we shift left
            self.0 << lsbs
        };

        Reg(n)
    }
}
