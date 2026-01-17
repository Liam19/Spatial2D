use crate::*;

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, RangeInclusive, Sub, SubAssign},
};

pub trait RectScalar:
    Copy
    + Clone
    + Display
    + Debug
    + PartialOrd
    + PartialEq
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
{
    type V2: Copy
        + Display
        + Debug
        + PartialEq
        + Add<Output = Self::V2>
        + Add<Self, Output = Self::V2>
        + AddAssign
        + Sub<Output = Self::V2>
        + Sub<Self, Output = Self::V2>
        + SubAssign
        + Div<Output = Self::V2>
        + Div<Self, Output = Self::V2>
        + DivAssign
        + Mul<Output = Self::V2>
        + Mul<Self, Output = Self::V2>
        + MulAssign;

    // type RangeInc: Iterator<Item = Self>;
    // type RangeEx: Iterator<Item = Self>;

    fn zero() -> Self;
    fn one() -> Self;
    fn two() -> Self;
    fn max() -> Self;
    fn min() -> Self;
    fn to_f32(self) -> f32;
    fn from_f32(float: f32) -> Self;

    // fn range_inclusive(start: Self, end: Self) -> Self::RangeInc;
    // fn range_exclusive(start: Self, end: Self) -> Self::RangeEx;

    // Vector2 construction and access
    fn v2_new(x: Self, y: Self) -> Self::V2;
    fn v2_x(v: Self::V2) -> Self;
    fn v2_y(v: Self::V2) -> Self;
    fn v2_x_mut(v: &mut Self::V2) -> &mut Self;
    fn v2_y_mut(v: &mut Self::V2) -> &mut Self;
    fn v2_min_element(v: Self::V2) -> Self;
    fn v2_max_element(v: Self::V2) -> Self;
    fn v2_element_product(v: Self::V2) -> Self;
}

impl RectScalar for i32 {
    type V2 = IVec2;

    // type RangeInc = RangeInclusive<i32>;

    // type RangeEx = Range<i32>;

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn two() -> Self {
        2
    }

    fn max() -> Self {
        i32::MAX
    }

    fn min() -> Self {
        i32::MIN
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn from_f32(float: f32) -> Self {
        float as i32
    }

    // fn range_inclusive(start: Self, end: Self) -> Self::RangeInc {
    //     start..=end
    // }

    // fn range_exclusive(start: Self, end: Self) -> Self::RangeEx {
    //     start..end
    // }

    fn v2_new(x: Self, y: Self) -> Self::V2 {
        IVec2::new(x, y)
    }

    fn v2_x(v: Self::V2) -> Self {
        v.x
    }

    fn v2_y(v: Self::V2) -> Self {
        v.y
    }

    fn v2_x_mut(v: &mut Self::V2) -> &mut Self {
        &mut v.x
    }

    fn v2_y_mut(v: &mut Self::V2) -> &mut Self {
        &mut v.y
    }

    fn v2_min_element(v: Self::V2) -> Self {
        v.min_element()
    }

    fn v2_max_element(v: Self::V2) -> Self {
        v.max_element()
    }

    fn v2_element_product(v: Self::V2) -> Self {
        v.element_product()
    }
}

impl RectScalar for u32 {
    type V2 = UVec2;

    // type RangeInc = RangeInclusive<u32>;

    // type RangeEx = Range<u32>;

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn two() -> Self {
        2
    }

    fn max() -> Self {
        u32::MAX
    }

    fn min() -> Self {
        u32::MIN
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn from_f32(float: f32) -> Self {
        float as u32
    }

    // fn range_inclusive(start: Self, end: Self) -> Self::RangeInc {
    //     start..=end
    // }

    // fn range_exclusive(start: Self, end: Self) -> Self::RangeEx {
    //     start..end
    // }

    fn v2_new(x: Self, y: Self) -> Self::V2 {
        UVec2::new(x, y)
    }

    fn v2_x(v: Self::V2) -> Self {
        v.x
    }

    fn v2_y(v: Self::V2) -> Self {
        v.y
    }

    fn v2_x_mut(v: &mut Self::V2) -> &mut Self {
        &mut v.x
    }

    fn v2_y_mut(v: &mut Self::V2) -> &mut Self {
        &mut v.y
    }

    fn v2_min_element(v: Self::V2) -> Self {
        v.min_element()
    }

    fn v2_max_element(v: Self::V2) -> Self {
        v.max_element()
    }

    fn v2_element_product(v: Self::V2) -> Self {
        v.element_product()
    }
}

impl RectScalar for f32 {
    type V2 = Vec2;

    // type RangeInc = RangeInclusive<f32>;

    // type RangeEx = Range<f32>;

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn two() -> Self {
        2.0
    }

    fn max() -> Self {
        f32::MAX
    }

    fn min() -> Self {
        f32::MIN
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn from_f32(float: f32) -> Self {
        float
    }

    // fn range_inclusive(start: Self, end: Self) -> Self::RangeInc {
    //     start..=end
    // }

    // fn range_exclusive(start: Self, end: Self) -> Self::RangeEx {
    //     start..end
    // }

    fn v2_new(x: Self, y: Self) -> Self::V2 {
        Vec2::new(x, y)
    }

    fn v2_x(v: Self::V2) -> Self {
        v.x
    }

    fn v2_y(v: Self::V2) -> Self {
        v.y
    }

    fn v2_x_mut(v: &mut Self::V2) -> &mut Self {
        &mut v.x
    }

    fn v2_y_mut(v: &mut Self::V2) -> &mut Self {
        &mut v.y
    }

    fn v2_min_element(v: Self::V2) -> Self {
        v.min_element()
    }

    fn v2_max_element(v: Self::V2) -> Self {
        v.max_element()
    }

    fn v2_element_product(v: Self::V2) -> Self {
        v.element_product()
    }
}
