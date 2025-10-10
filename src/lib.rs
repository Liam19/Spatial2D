mod direction;
mod matrix_module;
mod rect;
mod vec2_traits;

pub use direction::*;
pub use matrix_module::*;
pub use rect::*;
pub use vec2_traits::*;

pub(crate) use rng::*;

pub(crate) use hashbrown::{HashMap, HashSet};
pub(crate) use itertools::Itertools;
pub(crate) use std::fmt::Debug;

#[cfg(feature = "bevy-math")]
pub use bevy::math::{IVec2, UVec2, Vec2};

#[cfg(not(feature = "bevy-math"))]
pub use glam::{IVec2, UVec2, Vec2};

//TEMP:
use core::ops::Range;
/// Wraps a value around a specified range.
///
/// Given a value and a range, this function ensures that the value wraps around
/// to stay within the range. For example, a value of -1 with a range of `0..32`
/// will wrap to `31`, and a value of `32` will wrap to `0`.
///
/// # Examples
/// ```
/// # use spatial2d::wrap_i32;
/// let result = wrap_i32(-1, 0..32);
/// assert_eq!(result, 31);
///
/// let result = wrap_i32(32, 0..32);
/// assert_eq!(result, 0);
/// ```
#[inline]
pub(crate) fn wrap_i32(value: i32, range: Range<i32>) -> i32 {
    let size = range.end - range.start;

    ((value - range.start) % size + size) % size + range.start
}
