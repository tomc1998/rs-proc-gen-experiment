#![allow(dead_code)]
//! Fixed point arithmetic
//! # Why?
//! Fixed point arithmetic is useful for large open-world games, because it
//! means the distribution of represented value is uniform (meaning we don't
//! have the 'far lands' problem in minecraft so much, although we will have
//! other problems, only even more 'far' away)
//!
//! It also means we can fit floating point values in less bytes, good for cache
//! efficiency
//!
//! Not only that, but rounding modes on different CPUs makes floating point a
//! possible source of non-determinism, which means for games that use p2p
//! lockstep for multiplayer you can get de-syncs from simple floating point
//! arithmetic!

use num_integer::Roots;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

/// Bits after the binary point
pub const FPA_PREC : usize = 6;
/// The amount we have to multiply the number by to get the inner val (this
/// should be 2 ^ FPA_PREC)
pub const FPA_MUL : f32 = 64.0;

/**** 32-bit ****/

/// 32 bit fixed point number. Because of precision, this is limited to 26 bits
/// magnitude, i.e. no higher than 2^26.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fx32(pub i32);

impl Fx32 {
    pub fn new(val: f32) -> Fx32 { Fx32((val * FPA_MUL) as i32) }
    pub fn to_f32(&self) -> f32 { self.0 as f32 / FPA_MUL }
    pub fn to_fx16(&self) -> Fx16 { Fx16(self.0 as i16) }
    pub fn abs(self) -> Fx32 { Fx32(self.0.abs()) }
    pub fn powi(self, exp: usize) -> Fx32 {
        let mut total = self.0 as i64;
        for _ in 1..exp {
            total *= self.0 as i64;
            total /= FPA_MUL as i64;
        }
        Fx32(total as i32)
    }
    pub fn sqrt(self) -> Fx32 {
        Fx32((FPA_MUL as i64 * self.0 as i64).sqrt() as i32)
    }
}

impl Add<Fx32> for Fx32 { type Output = Fx32;  fn add(self, rhs: Self) -> Self::Output { Fx32(self.0 + rhs.0) } }
impl Sub<Fx32> for Fx32 { type Output = Fx32;  fn sub(self, rhs: Self) -> Self::Output { Fx32(self.0 - rhs.0) } }
impl Div<Fx32> for Fx32 { type Output = Fx32;  fn div(self, rhs: Self) -> Self::Output { Fx32(((self.0 as i64 * FPA_MUL as i64) as i64 / rhs.0 as i64) as i32) } }
impl Mul<Fx32> for Fx32 { type Output = Fx32;  fn mul(self, rhs: Self) -> Self::Output { Fx32((self.0 as i64 * rhs.0 as i64 / FPA_MUL as i64) as i32) } }
impl AddAssign<Fx32> for Fx32 { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
impl SubAssign<Fx32> for Fx32 { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
impl DivAssign<Fx32> for Fx32 { fn div_assign(&mut self, rhs: Self) { self.0 = (*self / rhs).0 } }
impl MulAssign<Fx32> for Fx32 { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
impl Add<Fx16> for Fx32 { type Output = Fx32;  fn add(self, rhs: Fx16) -> Self::Output { self + Fx32(rhs.0 as i32) } }
impl Sub<Fx16> for Fx32 { type Output = Fx32;  fn sub(self, rhs: Fx16) -> Self::Output { self - Fx32(rhs.0 as i32) } }
impl Div<Fx16> for Fx32 { type Output = Fx32;  fn div(self, rhs: Fx16) -> Self::Output { self / Fx32(rhs.0 as i32) } }
impl Mul<Fx16> for Fx32 { type Output = Fx32;  fn mul(self, rhs: Fx16) -> Self::Output { self * Fx32(rhs.0 as i32) } }
impl AddAssign<Fx16> for Fx32 { fn add_assign(&mut self, rhs: Fx16) { *self += Fx32(rhs.0 as i32) } }
impl SubAssign<Fx16> for Fx32 { fn sub_assign(&mut self, rhs: Fx16) { *self -= Fx32(rhs.0 as i32) } }
impl DivAssign<Fx16> for Fx32 { fn div_assign(&mut self, rhs: Fx16) { *self /= Fx32(rhs.0 as i32) } }
impl MulAssign<Fx16> for Fx32 { fn mul_assign(&mut self, rhs: Fx16) { *self *= Fx32(rhs.0 as i32) } }
impl Add<f32> for Fx32 { type Output = Fx32;  fn add(self, rhs: f32) -> Self::Output { Fx32::new(rhs) + self } }
impl Sub<f32> for Fx32 { type Output = Fx32;  fn sub(self, rhs: f32) -> Self::Output { self - Fx32::new(rhs) } }
impl Div<f32> for Fx32 { type Output = Fx32;  fn div(self, rhs: f32) -> Self::Output { self / Fx32::new(rhs) } }
impl Mul<f32> for Fx32 { type Output = Fx32;  fn mul(self, rhs: f32) -> Self::Output { Fx32::new(rhs) * self } }
impl AddAssign<f32> for Fx32 { fn add_assign(&mut self, rhs: f32) { *self += Fx32::new(rhs) } }
impl SubAssign<f32> for Fx32 { fn sub_assign(&mut self, rhs: f32) { *self -= Fx32::new(rhs) } }
impl DivAssign<f32> for Fx32 { fn div_assign(&mut self, rhs: f32) { *self /= Fx32::new(rhs) } }
impl MulAssign<f32> for Fx32 { fn mul_assign(&mut self, rhs: f32) { *self *= Fx32::new(rhs) } }
impl Neg for Fx32 { type Output = Self; fn neg(self) -> Self { Fx32(-self.0) } }

/**** 16-bit ****/

/// 16 bit fixed point number. Because of precision, this is limited to 10 bits
/// magnitude, i.e. no higher than 1024.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fx16(pub i16);

impl Fx16 {
    pub fn new(val: f32) -> Fx16 { Fx16((val * FPA_MUL) as i16) }
    pub fn to_f32(&self) -> f32 { self.0 as f32 / FPA_MUL }
    pub fn to_fx32(&self) -> Fx32 { Fx32(self.0 as i32) }
    pub fn abs(self) -> Fx16 { Fx16(self.0.abs()) }
    pub fn powi(self, exp: usize) -> Fx16 {
        let mut total = self.0 as i32;
        for _ in 1..exp {
            total *= self.0 as i32;
            total /= FPA_MUL as i32;
        }
        Fx16(total as i16)
    }
    pub fn sqrt(self) -> Fx16 {
        Fx16((FPA_MUL as i32 * self.0 as i32).sqrt() as i16)
    }
}
impl Add<Fx16> for Fx16 { type Output = Fx16; fn add(self, rhs: Self) -> Self::Output { Fx16(self.0 + rhs.0) } }
impl Sub<Fx16> for Fx16 { type Output = Fx16; fn sub(self, rhs: Self) -> Self::Output { Fx16(self.0 - rhs.0) } }
impl Div<Fx16> for Fx16 { type Output = Fx16; fn div(self, rhs: Self) -> Self::Output { Fx16(((self.0 as i32 * FPA_MUL as i32) as i32 / rhs.0 as i32) as i16) } }
impl Mul<Fx16> for Fx16 { type Output = Fx16; fn mul(self, rhs: Self) -> Self::Output { Fx16((self.0 as i32 * rhs.0 as i32 / FPA_MUL as i32) as i16) } }
impl AddAssign<Fx16> for Fx16 { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
impl SubAssign<Fx16> for Fx16 { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
impl DivAssign<Fx16> for Fx16 { fn div_assign(&mut self, rhs: Self) { self.0 = (*self / rhs).0 } }
impl MulAssign<Fx16> for Fx16 { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
impl Add<Fx32> for Fx16 { type Output = Fx16; fn add(self, rhs: Fx32) -> Self::Output { self + Fx16(rhs.0 as i16) } }
impl Sub<Fx32> for Fx16 { type Output = Fx16; fn sub(self, rhs: Fx32) -> Self::Output { self - Fx16(rhs.0 as i16) } }
impl Div<Fx32> for Fx16 { type Output = Fx16; fn div(self, rhs: Fx32) -> Self::Output { self / Fx16(rhs.0 as i16) } }
impl Mul<Fx32> for Fx16 { type Output = Fx16; fn mul(self, rhs: Fx32) -> Self::Output { self * Fx16(rhs.0 as i16) } }
impl AddAssign<Fx32> for Fx16 { fn add_assign(&mut self, rhs: Fx32) { *self += Fx16(rhs.0 as i16) } }
impl SubAssign<Fx32> for Fx16 { fn sub_assign(&mut self, rhs: Fx32) { *self -= Fx16(rhs.0 as i16) } }
impl DivAssign<Fx32> for Fx16 { fn div_assign(&mut self, rhs: Fx32) { *self /= Fx16(rhs.0 as i16) } }
impl MulAssign<Fx32> for Fx16 { fn mul_assign(&mut self, rhs: Fx32) { *self *= Fx16(rhs.0 as i16) } }
impl Add<f32> for Fx16 { type Output = Fx16; fn add(self, rhs: f32) -> Self::Output { Fx16::new(rhs) + self } }
impl Sub<f32> for Fx16 { type Output = Fx16; fn sub(self, rhs: f32) -> Self::Output { self - Fx16::new(rhs) } }
impl Div<f32> for Fx16 { type Output = Fx16; fn div(self, rhs: f32) -> Self::Output { self / Fx16::new(rhs) } }
impl Mul<f32> for Fx16 { type Output = Fx16; fn mul(self, rhs: f32) -> Self::Output { Fx16::new(rhs) * self } }
impl AddAssign<f32> for Fx16 { fn add_assign(&mut self, rhs: f32) { *self += Fx16::new(rhs) } }
impl SubAssign<f32> for Fx16 { fn sub_assign(&mut self, rhs: f32) { *self -= Fx16::new(rhs) } }
impl DivAssign<f32> for Fx16 { fn div_assign(&mut self, rhs: f32) { *self /= Fx16::new(rhs) } }
impl MulAssign<f32> for Fx16 { fn mul_assign(&mut self, rhs: f32) { *self *= Fx16::new(rhs) } }
impl Neg for Fx16 { type Output = Self; fn neg(self) -> Self { Fx16(-self.0) } }


#[cfg(test)]
mod tests {
    use super::*;
    use test;
    use std::mem::size_of;

    #[test]
    fn test_fpa_consts() {
        assert_eq!(2.0f32.powi(FPA_PREC as i32), FPA_MUL)
    }

    #[test]
    fn test_fpa32() {
        assert_eq!(size_of::<Fx32>(), 4);
        let mut foo = Fx32::new(64.0);
        let bar = Fx32::new(150.0);
        foo += 32.0;
        foo -= 16.0;
        foo *= 4.0;
        foo /= 2.0;
        assert_eq!(foo.to_f32(), 160.0);
        assert_eq!(foo, bar + 10.0);
        assert_eq!((-foo).to_f32(), -160.0);
        assert!(foo > bar);
        assert_eq!(Fx32::new(2.0).powi(2).to_f32(), 4.0);
        assert_eq!(Fx32::new(-2.0).powi(2).to_f32(), 4.0);
        assert_eq!(Fx32::new(4.0).sqrt().to_f32(), 2.0);
        assert_eq!(Fx32::new(2.25).sqrt().to_f32(), 1.5);
    }

    #[test]
    fn test_fpa16() {
        assert_eq!(size_of::<Fx16>(), 2);
        let mut foo = Fx16::new(64.0);
        let bar = Fx16::new(150.0);
        foo += 32.0;
        println!("{:?}", foo);
        foo -= 16.0;
        println!("{:?}", foo);
        foo /= 2.0;
        println!("{:?}", foo);
        foo *= 4.0;
        println!("{:?}", foo);
        assert_eq!(foo.to_f32(), 160.0);
        assert_eq!(foo, bar + 10.0);
        assert_eq!((-foo).to_f32(), -160.0);
        assert!(foo > bar);
        assert_eq!(Fx16::new(2.0).powi(2).to_f32(), 4.0);
        assert_eq!(Fx16::new(4.0).sqrt().to_f32(), 2.0);
        assert_eq!(Fx16::new(2.25).sqrt().to_f32(), 1.5);
    }

    #[bench]
    fn bench_1000_f32_mul(b: &mut test::Bencher) {
        let foo = 34.1029;
        let bar = 12.2384;
        b.iter(|| {
            for _ in 0..1000 {
                test::black_box(foo * bar);
            }
        });
    }

    #[bench]
    fn bench_1000_fx32_mul(b: &mut test::Bencher) {
        let foo = Fx32::new(34.1029);
        let bar = Fx32::new(12.2384);
        b.iter(|| {
            for _ in 0..1000 {
                test::black_box(foo * bar);
            }
        });
    }

    #[bench]
    fn bench_1000_fx16_mul(b: &mut test::Bencher) {
        let foo = Fx16::new(34.1029);
        let bar = Fx16::new(12.2384);
        b.iter(|| {
            for _ in 0..1000 {
                test::black_box(foo * bar);
            }
        });
    }
}
