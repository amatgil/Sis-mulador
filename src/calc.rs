use std::ops::{Add, Shl, Sub};

use crate::execute::Reg;

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

        let n = if sign > 0 { // rhs is negative, we shift right
            (self.0 / 2) >> (!lsbs & 0x000F)
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
        //crate::print_info(&format!("{} shl {}. shift amount is: {} towards {}", self.0, rhs.0, lsbs, sign));
        let n = if sign > 0 { // rhs is negative, we shift right
            (self.0 / 2) >> (!lsbs & 0x000F)  // We divide by two because the fifth bit also counts as a right shift
        } else { // rhs is positive, we shift left
            self.0 << lsbs
        };
        crate::print_info(&format!("{} shl {} is: {}. lsbs: {:X}", self.0, rhs.0, n, lsbs));

        Reg(n)
    }
}
