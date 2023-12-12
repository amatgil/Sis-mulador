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

impl Shl for Reg {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        let lsbs = rhs.0 & 0b01111;
        let sign = rhs.0 & 0b10000;
        // rhs is negative, we shift right
        let n = if sign > 0 {
            self.0 >> lsbs
        } else {
            // rhs is positive, we shift left
            self.0 << lsbs
        };

        Reg(n)
    }
}
