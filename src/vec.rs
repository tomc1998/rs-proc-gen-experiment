#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec32 {
    pub x: f32,
    pub y: f32
}
impl Vec32 {
    pub fn new(x: f32, y: f32) -> Vec32 {Vec32 {x, y}}
    pub fn zero() -> Vec32 { Vec32 { x: 0.0, y: 0.0 } }
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn nor(self) -> Vec32 {self / self.len()}
    pub fn angle(self) -> f32 { self.y.atan2(self.x) }
}
impl Neg for Vec32 { type Output = Self; fn neg(self) -> Self { Vec32::new(-self.x, -self.y) } }
impl Add<Vec32> for Vec32 {
    type Output = Vec32;
    fn add(self, rhs: Vec32) -> Self::Output { Vec32 { x: self.x + rhs.x, y: self.y + rhs.y } }
}
impl Sub<Vec32> for Vec32 {
    type Output = Vec32;
    fn sub(self, rhs: Vec32) -> Self::Output { Vec32 { x: self.x - rhs.x, y: self.y - rhs.y } }
}
impl Div<f32> for Vec32 {
    type Output = Vec32;
    fn div(self, rhs: f32) -> Self::Output { Vec32 { x: self.x / rhs, y: self.y / rhs } }
}
impl Mul<f32> for Vec32 {
    type Output = Vec32;
    fn mul(self, rhs: f32) -> Self::Output { Vec32 { x: self.x * rhs, y: self.y * rhs } }
}
impl AddAssign<Vec32> for Vec32 {
    fn add_assign(&mut self, rhs: Self) {self.x += rhs.x; self.y += rhs.y;}
}
impl SubAssign<Vec32> for Vec32 {
    fn sub_assign(&mut self, rhs: Self) {self.x -= rhs.x; self.y -= rhs.y;}
}
impl MulAssign<f32> for Vec32 {
    fn mul_assign(&mut self, rhs: f32) {self.x *= rhs; self.y *= rhs;}
}
impl DivAssign<f32> for Vec32 {
    fn div_assign(&mut self, rhs: f32) {self.x /= rhs; self.y /= rhs;}
}
