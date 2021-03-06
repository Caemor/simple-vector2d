#![warn(missing_docs)]
//! Simple and generic implementation of 2D vectors
//!
//! Intended for use in 2D game engines

extern crate num_traits;
#[cfg(feature="rustc-serialize")]
extern crate rustc_serialize;

#[cfg(feature="serde_derive")]
#[cfg_attr(feature="serde_derive", macro_use)]
extern crate serde_derive;

use num_traits::Float;

/// Representation of a mathematical vector e.g. a position or velocity
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature="rustc-serialize", derive(RustcDecodable, RustcEncodable))]
#[cfg_attr(feature="serde_derive", derive(Serialize, Deserialize))]
pub struct Vector2<T>(pub T, pub T);

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::convert::From;

/// Constants for common vectors
pub mod consts{
    use super::Vector2;

    /// The zero vector
    pub const ZERO_F32: Vector2<f32> = Vector2(0., 0.);
    /// A unit vector pointing upwards
    pub const UP_F32: Vector2<f32> = Vector2(0., 1.);
    /// A unit vector pointing downwards
    pub const DOWN_F32: Vector2<f32> = Vector2(0., -1.);
    /// A unit vector pointing to the right
    pub const RIGHT_F32: Vector2<f32> = Vector2(1., 0.);
    /// A unit vector pointing to the left
    pub const LEFT_F32: Vector2<f32> = Vector2(-1., 0.);

    /// The zero vector
    pub const ZERO_F64: Vector2<f64> = Vector2(0., 0.);
    /// A unit vector pointing upwards
    pub const UP_F64: Vector2<f64> = Vector2(0., 1.);
    /// A unit vector pointing downwards
    pub const DOWN_F64: Vector2<f64> = Vector2(0., -1.);
    /// A unit vector pointing to the right
    pub const RIGHT_F64: Vector2<f64> = Vector2(1., 0.);
    /// A unit vector pointing to the left
    pub const LEFT_F64: Vector2<f64> = Vector2(-1., 0.);
}

impl<T: Float> Vector2<T>{
    /// Creates a new unit vector in a specific direction
    pub fn unit_vector(direction: T) -> Self{
        let (y, x) = direction.sin_cos();
        Vector2(x, y)
    }
    /// Normalises the vector
    pub fn normalise(self) -> Self{
        self / self.length()
    }
    /// Returns the magnitude/length of the vector
    pub fn length(self) -> T{
        // This is apparently faster than using hypot
        self.length_squared().sqrt()
    }
    /// Returns the magnitude/length of the vector squared
    pub fn length_squared(self) -> T{
        self.0.powi(2) + self.1.powi(2)
    }
    /// Returns direction the vector is pointing
    pub fn direction(self) -> T{
        self.1.atan2(self.0)
    }
    /// Returns direction towards another vector
    pub fn direction_to(self, other: Self) -> T{
        (other-self).direction()
    }
    /// Returns the distance betweens two vectors
    pub fn distance_to(self, other: Self) -> T{
        (other-self).length()
    }
    /// Returns the distance betweens two vectors
    pub fn distance_to_squared(self, other: Self) -> T{
        (other-self).length_squared()
    }
    /// Returns `true` if either component is `NaN`.
    pub fn is_any_nan(&self) -> bool{
        self.0.is_nan() || self.1.is_nan()
    }
    /// Returns `true` if either component is positive or negative infinity.
    pub fn is_any_infinite(&self) -> bool{
        self.0.is_infinite() || self.1.is_infinite()
    }
    /// Returns `true` if both components are neither infinite nor `NaN`.
    pub fn is_all_finite(&self) -> bool{
        self.0.is_finite() && self.1.is_finite()
    }
    /// Returns `true` if both components are neither zero, infinite, subnormal nor `NaN`.
    pub fn is_all_normal(&self) -> bool{
        self.0.is_normal() && self.1.is_normal()
    }
}

macro_rules! impl_for {
    ($($t:ty)*) => {$(
        impl Mul<Vector2<$t>> for $t{
            type Output = Vector2<$t>;

            fn mul(self, rhs: Vector2<$t>) -> Vector2<$t>{
                Vector2(self * rhs.0, self * rhs.1)
            }
        }
        impl Div<Vector2<$t>> for $t{
            type Output = Vector2<$t>;

            fn div(self, rhs: Vector2<$t>) -> Vector2<$t>{
                Vector2(self / rhs.0, self / rhs.1)
            }
        }
    )*};
}impl_for!{f32 f64}

impl<T> Vector2<T> {
    /// Returns the normal vector (aka. hat vector) of this vector i.e. a perpendicular vector
    ///
    /// Not to be confused with `normalise` which returns a unit vector
    ///
    /// Defined as (-y, x)
    pub fn normal(self) -> Self
    where T: Neg<Output=T> {
        let Vector2(x, y) = self;
        Vector2(-y, x)
    }
    /// Returns the dot product of two vectors
    pub fn dot(self, other: Self) -> <<T as Mul>::Output as Add>::Output
    where T: Mul, <T as Mul>::Output: Add{
        self.0 * other.0 + self.1 * other.1
    }
    /// Returns the determinant of two vectors
    pub fn det(self, other: Self) -> <<T as Mul>::Output as Sub>::Output
    where T: Mul, <T as Mul>::Output: Sub {
        self.0 * other.1 - self.1 * other.0
    }
}

impl<T: Add> Add for Vector2<T>{
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Self) -> Self::Output{
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub> Sub for Vector2<T>{
    type Output = Vector2<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output{
        Vector2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: AddAssign> AddAssign for Vector2<T>{
    fn add_assign(&mut self, rhs: Self){
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: SubAssign> SubAssign for Vector2<T>{
    fn sub_assign(&mut self, rhs: Self){
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T>{
    fn mul_assign(&mut self, rhs: T){
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector2<T>{
    fn div_assign(&mut self, rhs: T){
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl<T: Mul + Copy> Mul<T> for Vector2<T>{
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: T) -> Self::Output{
        Vector2(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Div + Copy> Div<T> for Vector2<T>{
    type Output = Vector2<T::Output>;

    fn div(self, rhs: T) -> Self::Output{
        Vector2(self.0/rhs, self.1/rhs)
    }
}

impl<T: Neg> Neg for Vector2<T>{
    type Output = Vector2<T::Output>;

    fn neg(self) -> Self::Output{
        Vector2(-self.0, -self.1)
    }
}

impl<T> Into<[T; 2]> for Vector2<T>{
    #[inline]
    fn into(self) -> [T; 2]{
        [self.0, self.1]
    }
}

impl<T: Copy> From<[T; 2]> for Vector2<T>{
    #[inline]
    fn from(array: [T; 2]) -> Self{
        Vector2(array[0], array[1])
    }
}

impl<T> Into<(T, T)> for Vector2<T>{
    #[inline]
    fn into(self) -> (T, T){
        (self.0, self.1)
    }
}

impl<T> From<(T, T)> for Vector2<T>{
    #[inline]
    fn from(tuple: (T, T)) -> Self{
        Vector2(tuple.0, tuple.1)
    }
}
