//! Fixed point arithmetic

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Deref};
use std::cmp::{Ord, PartialOrd, Eq, PartialEq, Ordering};

/// Bits after the binary point
const FPA_PREC : usize = 2;
/// The amount we have to multiply the number by to get the inner val (this
/// should be 2 ^ FPA_PREC)
const FPA_MUL : f32 = 4.0;

/// 32 bit fixed point number
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fx32(pub i32);

impl Fx32 {
    pub fn new(val: f32) -> Fx32 { Fx32((val * FPA_MUL) as i32) }
    pub fn to_f32(&self) -> f32 { self.0 as f32 / FPA_MUL }
}

impl Add<Fx32> for Fx32 {
    type Output = Fx32;
    fn add(self, rhs: Self) -> Self::Output { Fx32(self.0 + rhs.0) }
}

impl Sub<Fx32> for Fx32 {
    type Output = Fx32;
    fn sub(self, rhs: Self) -> Self::Output { Fx32(self.0 - rhs.0) }
}

impl Div<Fx32> for Fx32 {
    type Output = Fx32;
    fn div(self, rhs: Self) -> Self::Output { Fx32(self.0 / rhs.0) }
}

impl Mul<Fx32> for Fx32 {
    type Output = Fx32;
    fn mul(self, rhs: Self) -> Self::Output { Fx32(self.0 * rhs.0) }
}

impl AddAssign<Fx32> for Fx32 {
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0 }
}

impl SubAssign<Fx32> for Fx32 {
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0 }
}

impl DivAssign<Fx32> for Fx32 {
    fn div_assign(&mut self, rhs: Self) { self.0 /= rhs.0 }
}

impl MulAssign<Fx32> for Fx32 {
    fn mul_assign(&mut self, rhs: Self) { self.0 *= rhs.0 }
}

impl Add<f32> for Fx32 {
    type Output = Fx32;
    fn add(self, rhs: f32) -> Self::Output { Fx32::new(rhs) + self }
}

impl Sub<f32> for Fx32 {
    type Output = Fx32;
    fn sub(self, rhs: f32) -> Self::Output { self - Fx32::new(rhs) }
}

impl Div<f32> for Fx32 {
    type Output = Fx32;
    fn div(self, rhs: f32) -> Self::Output { self / Fx32::new(rhs) }
}

impl Mul<f32> for Fx32 {
    type Output = Fx32;
    fn mul(self, rhs: f32) -> Self::Output { Fx32::new(rhs) * self }
}

impl AddAssign<f32> for Fx32 {
    fn add_assign(&mut self, rhs: f32) { *self += Fx32::new(rhs) }
}

impl SubAssign<f32> for Fx32 {
    fn sub_assign(&mut self, rhs: f32) { *self -= Fx32::new(rhs) }
}

impl DivAssign<f32> for Fx32 {
    fn div_assign(&mut self, rhs: f32) { *self /= Fx32::new(rhs) }
}

impl MulAssign<f32> for Fx32 {
    fn mul_assign(&mut self, rhs: f32) { *self *= Fx32::new(rhs) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_fpa_consts() {
        assert_eq!(2.0f32.powi(FPA_PREC as i32), FPA_MUL)
    }

    #[test]
    fn test_fpa() {
        let mut foo = Fx32::new(64.0);
        let mut bar = Fx32::new(150.0);
        foo += 32.0;
        foo -= 16.0;
        foo *= 4.0;
        foo /= 2.0;
        assert_eq!(foo.to_f32(), 160.0);
        assert_eq!(foo, bar + 10.0);
        assert!(foo > bar);
    }
}
