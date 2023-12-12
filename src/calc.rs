use std::ops::{Add, Sub, Shl};

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
        let lsbs: Vec<bool> = rhs.0.to_string().chars().rev().take(5).map(|b| b == '1').collect();
        dbg!(self.0, rhs.0, lsbs);
    }
}
