#![allow(dead_code)]
//! Fixed point vectors

use num_integer::Roots;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use fpa::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec16 {
    pub x: Fx16,
    pub y: Fx16
}
impl Vec16 {
    pub fn new(x: Fx16, y: Fx16) -> Vec16 {Vec16 {x, y}}
    pub fn zero() -> Vec16 { Vec16 { x: Fx16::new(0.0), y: Fx16::new(0.0) } }
    pub fn to_32(&self) -> Vec32 {Vec32::new(self.x.to_fx32(), self.y.to_fx32())}
    pub fn len(&self) -> Fx16 {
        Fx16((self.x.0 as i32 * self.x.0 as i32 +
              self.y.0 as i32 * self.y.0 as i32).sqrt() as i16)
    }
    pub fn nor(self) -> Vec16 {self / self.len()}
    /// WARNING: This is not a fixed point operation.
    pub fn angle(self) -> f32 { self.y.to_f32().atan2(self.x.to_f32()) }
}
impl Add<Vec16> for Vec16 {
    type Output = Vec16;
    fn add(self, rhs: Self) -> Self::Output { Vec16 { x: self.x + rhs.x, y: self.y + rhs.y } }
}
impl Sub<Vec16> for Vec16 {
    type Output = Vec16;
    fn sub(self, rhs: Self) -> Self::Output { Vec16 { x: self.x - rhs.x, y: self.y - rhs.y } }
}
impl Add<Vec32> for Vec16 {
    type Output = Vec16;
    fn add(self, rhs: Vec32) -> Self::Output { Vec16 { x: self.x + rhs.x, y: self.y + rhs.y } }
}
impl Sub<Vec32> for Vec16 {
    type Output = Vec16;
    fn sub(self, rhs: Vec32) -> Self::Output { Vec16 { x: self.x - rhs.x, y: self.y - rhs.y } }
}
impl Div<Fx16> for Vec16 {
    type Output = Vec16;
    fn div(self, rhs: Fx16) -> Self::Output { Vec16 { x: self.x / rhs, y: self.y / rhs } }
}
impl Mul<Fx16> for Vec16 {
    type Output = Vec16;
    fn mul(self, rhs: Fx16) -> Self::Output { Vec16 { x: self.x * rhs, y: self.y * rhs } }
}
impl Div<Fx32> for Vec16 {
    type Output = Vec16;
    fn div(self, rhs: Fx32) -> Self::Output { Vec16 { x: self.x / rhs, y: self.y / rhs } }
}
impl Mul<Fx32> for Vec16 {
    type Output = Vec16;
    fn mul(self, rhs: Fx32) -> Self::Output { Vec16 { x: self.x * rhs, y: self.y * rhs } }
}
impl AddAssign<Vec16> for Vec16 {
    fn add_assign(&mut self, rhs: Self) {self.x += rhs.x; self.y += rhs.y;}
}
impl SubAssign<Vec16> for Vec16 {
    fn sub_assign(&mut self, rhs: Self) {self.x -= rhs.x; self.y -= rhs.y;}
}
impl MulAssign<Fx16> for Vec16 {
    fn mul_assign(&mut self, rhs: Fx16) {self.x *= rhs; self.y *= rhs;}
}
impl DivAssign<Fx16> for Vec16 {
    fn div_assign(&mut self, rhs: Fx16) {self.x /= rhs; self.y /= rhs;}
}
impl MulAssign<Fx32> for Vec16 {
    fn mul_assign(&mut self, rhs: Fx32) {self.x *= rhs; self.y *= rhs;}
}
impl DivAssign<Fx32> for Vec16 {
    fn div_assign(&mut self, rhs: Fx32) {self.x /= rhs; self.y /= rhs;}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec32 {
    pub x: Fx32,
    pub y: Fx32
}
impl Vec32 {
    pub fn new(x: Fx32, y: Fx32) -> Vec32 {Vec32 {x, y}}
    pub fn zero() -> Vec32 { Vec32 { x: Fx32::new(0.0), y: Fx32::new(0.0) } }
    pub fn to_16(&self) -> Vec16 {Vec16::new(self.x.to_fx16(), self.y.to_fx16())}
    pub fn len(&self) -> Fx32 {
        Fx32((self.x.0 as i64 * self.x.0 as i64 +
              self.y.0 as i64 * self.y.0 as i64).sqrt() as i32)
    }
    pub fn nor(self) -> Vec32 {self / self.len()}
    pub fn angle(self) -> f32 { self.y.to_f32().atan2(self.x.to_f32()) }
}
impl Add<Vec32> for Vec32 {
    type Output = Vec32;
    fn add(self, rhs: Self) -> Self::Output { Vec32 { x: self.x + rhs.x, y: self.y + rhs.y } }
}
impl Sub<Vec32> for Vec32 {
    type Output = Vec32;
    fn sub(self, rhs: Self) -> Self::Output { Vec32 { x: self.x - rhs.x, y: self.y - rhs.y } }
}
impl Add<Vec16> for Vec32 {
    type Output = Vec32;
    fn add(self, rhs: Vec16) -> Self::Output { Vec32 { x: self.x + rhs.x, y: self.y + rhs.y } }
}
impl Sub<Vec16> for Vec32 {
    type Output = Vec32;
    fn sub(self, rhs: Vec16) -> Self::Output { Vec32 { x: self.x - rhs.x, y: self.y - rhs.y } }
}
impl Div<Fx32> for Vec32 {
    type Output = Vec32;
    fn div(self, rhs: Fx32) -> Self::Output { Vec32 { x: self.x / rhs, y: self.y / rhs } }
}
impl Mul<Fx32> for Vec32 {
    type Output = Vec32;
    fn mul(self, rhs: Fx32) -> Self::Output { Vec32 { x: self.x * rhs, y: self.y * rhs } }
}
impl Div<Fx16> for Vec32 {
    type Output = Vec32;
    fn div(self, rhs: Fx16) -> Self::Output { Vec32 { x: self.x / rhs, y: self.y / rhs } }
}
impl Mul<Fx16> for Vec32 {
    type Output = Vec32;
    fn mul(self, rhs: Fx16) -> Self::Output { Vec32 { x: self.x * rhs, y: self.y * rhs } }
}
impl AddAssign<Vec32> for Vec32 {
    fn add_assign(&mut self, rhs: Self) {self.x += rhs.x; self.y += rhs.y;}
}
impl SubAssign<Vec32> for Vec32 {
    fn sub_assign(&mut self, rhs: Self) {self.x -= rhs.x; self.y -= rhs.y;}
}
impl MulAssign<Fx32> for Vec32 {
    fn mul_assign(&mut self, rhs: Fx32) {self.x *= rhs; self.y *= rhs;}
}
impl DivAssign<Fx32> for Vec32 {
    fn div_assign(&mut self, rhs: Fx32) {self.x /= rhs; self.y /= rhs;}
}
impl MulAssign<Fx16> for Vec32 {
    fn mul_assign(&mut self, rhs: Fx16) {self.x *= rhs; self.y *= rhs;}
}
impl DivAssign<Fx16> for Vec32 {
    fn div_assign(&mut self, rhs: Fx16) {self.x /= rhs; self.y /= rhs;}
}
