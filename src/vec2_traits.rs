use crate::*;

/// A trait for retrieving all positions represented by a 2D Vector
pub trait Positions {
    type V;

    /// Returns a vector of all positions in a 2D Vector
    ///
    /// Positions are in left-to-right & top-to-bottom order
    /// ## Example
    /// ```
    /// # use bevy::prelude::IVec2;
    /// # use utils::Positions;
    /// let vector = IVec2::new(2, 3);
    ///
    /// assert_eq!(
    ///     vector.positions(),
    ///     vec![
    ///        IVec2::new(0, 0),
    ///        IVec2::new(1, 0),
    ///        IVec2::new(0, 1),
    ///        IVec2::new(1, 1),
    ///        IVec2::new(0, 2),
    ///        IVec2::new(1, 2),
    ///     ]
    /// );
    /// ```
    fn positions(&self) -> Vec<Self::V>;

    /// Inclusive variant of `positions()`
    /// ## Example
    /// ```
    /// # use bevy::prelude::IVec2;
    /// # use utils::Positions;
    /// let vector = IVec2::new(1, 2);
    ///
    /// assert_eq!(
    ///     vector.positions_inclusive(),
    ///     vec![
    ///        IVec2::new(0, 0),
    ///        IVec2::new(1, 0),
    ///        IVec2::new(0, 1),
    ///        IVec2::new(1, 1),
    ///        IVec2::new(0, 2),
    ///        IVec2::new(1, 2),
    ///     ]
    /// );
    /// ```
    fn positions_inclusive(&self) -> Vec<Self::V>;
}

impl Positions for IVec2 {
    type V = Self;

    #[inline]
    fn positions(&self) -> Vec<Self::V> {
        let mut positions = Vec::new();

        for y in 0..self.y {
            for x in 0..self.x {
                positions.push(Self::new(x, y));
            }
        }

        positions
    }

    #[inline]
    fn positions_inclusive(&self) -> Vec<Self::V> {
        let mut positions = Vec::new();

        for y in 0..=self.y {
            for x in 0..=self.x {
                positions.push(Self::new(x, y));
            }
        }

        positions
    }
}

impl Positions for UVec2 {
    type V = Self;

    #[inline]
    fn positions(&self) -> Vec<Self::V> {
        let mut positions = Vec::new();

        for y in 0..self.y {
            for x in 0..self.x {
                positions.push(Self::new(x, y));
            }
        }

        positions
    }

    #[inline]
    fn positions_inclusive(&self) -> Vec<Self::V> {
        let mut positions = Vec::new();

        for y in 0..=self.y {
            for x in 0..=self.x {
                positions.push(Self::new(x, y));
            }
        }

        positions
    }
}

/// A trait for retrieving all neighbouring positions of a 2D vector
pub trait Neighbours {
    type V;

    fn neighbours(&self) -> [Self::V; 8];

    fn neighbours_no_diag(&self) -> [Self::V; 4];

    fn neighbours_diag_only(&self) -> [Self::V; 4];

    fn neighbours_length(&self, length: u32) -> [Self::V; 8];

    fn neighbours_no_diag_length(&self, length: u32) -> [Self::V; 4];

    fn neighbours_diag_only_length(&self, length: u32) -> [Self::V; 4];

    fn neighbours_square(&self, radius: u32) -> Vec<Self::V>;

    fn neighbours_circle(&self, radius: u32) -> Vec<Self::V>;
}

impl Neighbours for IVec2 {
    type V = Self;

    #[inline]
    fn neighbours(&self) -> [Self::V; 8] {
        [
            self + IVec2::new(0, -1),  // Up
            self + IVec2::new(0, 1),   // Down
            self + IVec2::new(-1, 0),  // Left
            self + IVec2::new(1, 0),   // Right
            self + IVec2::new(-1, -1), // UpLeft
            self + IVec2::new(1, -1),  // UpRight
            self + IVec2::new(-1, 1),  // DownLeft
            self + IVec2::new(1, 1),   // DownRight
        ]
    }

    #[inline]
    fn neighbours_no_diag(&self) -> [Self::V; 4] {
        [
            self + IVec2::new(0, -1), // Up
            self + IVec2::new(0, 1),  // Down
            self + IVec2::new(-1, 0), // Left
            self + IVec2::new(1, 0),  // Right
        ]
    }

    #[inline]
    fn neighbours_diag_only(&self) -> [Self::V; 4] {
        [
            self + IVec2::new(-1, -1), // UpLeft
            self + IVec2::new(1, -1),  // UpRight
            self + IVec2::new(-1, 1),  // DownLeft
            self + IVec2::new(1, 1),   // DownRight
        ]
    }

    fn neighbours_length(&self, length: u32) -> [Self::V; 8] {
        let v = length as i32;

        [
            self + IVec2::new(0, -v),  // Up
            self + IVec2::new(0, v),   // Down
            self + IVec2::new(-v, 0),  // Left
            self + IVec2::new(v, 0),   // Right
            self + IVec2::new(-v, -v), // UpLeft
            self + IVec2::new(v, -v),  // UpRight
            self + IVec2::new(-v, v),  // DownLeft
            self + IVec2::new(v, v),   // DownRight
        ]
    }

    fn neighbours_no_diag_length(&self, length: u32) -> [Self::V; 4] {
        let v = length as i32;

        [
            self + IVec2::new(0, -v), // Up
            self + IVec2::new(0, v),  // Down
            self + IVec2::new(-v, 0), // Left
            self + IVec2::new(v, 0),  // Right
        ]
    }

    fn neighbours_diag_only_length(&self, length: u32) -> [Self::V; 4] {
        let v = length as i32;

        [
            self + IVec2::new(-v, -v), // UpLeft
            self + IVec2::new(v, -v),  // UpRight
            self + IVec2::new(-v, v),  // DownLeft
            self + IVec2::new(v, v),   // DownRight
        ]
    }

    #[inline]
    fn neighbours_square(&self, radius: u32) -> Vec<Self::V> {
        let r = radius as i32;

        IVec2::splat(r * 2)
            .positions_inclusive()
            .into_iter()
            .map(|pos| self + (pos - IVec2::splat(r)))
            .filter(|pos| pos != self)
            .collect::<Vec<IVec2>>()
    }

    #[inline]
    fn neighbours_circle(&self, radius: u32) -> Vec<Self::V> {
        let r = radius as i32;

        IVec2::splat(r * 2)
            .positions_inclusive()
            .into_iter()
            .map(|pos| self + (pos - IVec2::splat(r)))
            .filter(|pos| {
                if pos == self {
                    return false;
                }

                let dist = self.as_vec2().distance(pos.as_vec2());

                if dist > radius as f32 {
                    return false;
                }

                true
            })
            .collect::<Vec<IVec2>>()
    }
}

impl Neighbours for UVec2 {
    type V = Self;

    #[inline]
    fn neighbours(&self) -> [Self::V; 8] {
        [
            self - UVec2::new(1, 1),                    // Top left
            self - UVec2::new(0, 1),                    // Top
            self - UVec2::new(0, 1) + UVec2::new(1, 0), // Top right
            self - UVec2::new(1, 0),                    // Left
            self + UVec2::new(1, 0),                    // Right
            self + UVec2::new(0, 1) - UVec2::new(1, 0), // Bottom left
            self + UVec2::new(0, 1),                    // Bottom
            self + UVec2::new(1, 1),                    // Bottom right
        ]
    }

    #[inline]
    fn neighbours_no_diag(&self) -> [Self::V; 4] {
        [
            self - UVec2::new(0, 1),
            self - UVec2::new(1, 0),
            self + UVec2::new(1, 0),
            self + UVec2::new(0, 1),
        ]
    }

    #[inline]
    fn neighbours_diag_only(&self) -> [Self::V; 4] {
        [
            self - UVec2::new(1, 1),                    // Top left
            self - UVec2::new(0, 1) + UVec2::new(1, 0), // Top right
            self + UVec2::new(0, 1) - UVec2::new(1, 0), // Bottom left
            self + UVec2::new(1, 1),                    // Bottom right
        ]
    }

    #[inline]
    fn neighbours_length(&self, length: u32) -> [Self::V; 8] {
        [
            self - UVec2::new(length, length),                    // Top left
            self - UVec2::new(0, length),                         // Top
            self - UVec2::new(0, length) + UVec2::new(length, 0), // Top right
            self - UVec2::new(length, 0),                         // Left
            self + UVec2::new(length, 0),                         // Right
            self + UVec2::new(0, length) - UVec2::new(length, 0), // Bottom left
            self + UVec2::new(0, length),                         // Bottom
            self + UVec2::new(length, length),                    // Bottom right
        ]
    }

    #[inline]
    fn neighbours_no_diag_length(&self, length: u32) -> [Self::V; 4] {
        [
            self - UVec2::new(0, length),
            self - UVec2::new(length, 0),
            self + UVec2::new(length, 0),
            self + UVec2::new(0, length),
        ]
    }

    #[inline]
    fn neighbours_diag_only_length(&self, length: u32) -> [Self::V; 4] {
        [
            self - UVec2::new(length, length),                    // Top left
            self - UVec2::new(0, length) + UVec2::new(length, 0), // Top right
            self + UVec2::new(0, length) - UVec2::new(length, 0), // Bottom left
            self + UVec2::new(length, length),                    // Bottom right
        ]
    }

    #[inline]
    fn neighbours_square(&self, radius: u32) -> Vec<Self::V> {
        let r = radius as i32;
        let square = IVec2::splat(r);

        IVec2::splat(r * 2)
            .positions_inclusive()
            .into_iter()
            .map(|pos| (self.as_ivec2() + (pos - square)).as_uvec2())
            .filter(|pos| pos != self)
            .collect::<Vec<UVec2>>()
    }

    #[inline]
    fn neighbours_circle(&self, radius: u32) -> Vec<Self::V> {
        let r = radius as i32;
        let square = IVec2::splat(r);

        IVec2::splat(r * 2)
            .positions_inclusive()
            .into_iter()
            .map(|pos| (self.as_ivec2() + (pos - square)).as_uvec2())
            .filter(|pos| {
                if pos == self {
                    return false;
                }

                let dist = self.as_vec2().distance(pos.as_vec2());

                if dist > radius as f32 {
                    return false;
                }

                true
            })
            .collect::<Vec<UVec2>>()
    }
}

/// Same as the `Neighbours` trait but for UVec2 only
///
/// Checks for invalid positions (UVec2 can't contain negative values)
pub trait NeighboursChecked {
    type V;

    fn neighbours_checked(&self) -> Vec<Self::V>;

    fn neighbours_checked_no_diag(&self) -> Vec<Self::V>;

    fn neighbours_checked_diag_only(&self) -> Vec<Self::V>;

    fn neighbours_checked_length(&self, length: u32) -> Vec<Self::V>;

    fn neighbours_checked_no_diag_length(&self, length: u32) -> Vec<Self::V>;

    fn neighbours_checked_diag_only_length(&self, length: u32) -> Vec<Self::V>;

    fn neighbours_checked_square(&self, radius: u32) -> Vec<Self::V>;

    fn neighbours_checked_circle(&self, radius: u32) -> Vec<Self::V>;
}

impl NeighboursChecked for UVec2 {
    type V = Self;

    #[inline]
    fn neighbours_checked(&self) -> Vec<Self::V> {
        let mut neighbours = Vec::new();

        // Top left
        if self.x > 0 && self.y > 0 {
            neighbours.push(self - UVec2::new(1, 1));
        }
        // Top
        if self.y > 0 {
            neighbours.push(self - UVec2::new(0, 1));
        }
        // Top right
        if self.y > 0 {
            neighbours.push(self - UVec2::new(0, 1) + UVec2::new(1, 0));
        }
        // Left
        if self.x > 0 {
            neighbours.push(self - UVec2::new(1, 0));
        }

        // Right
        neighbours.push(self + UVec2::new(1, 0));

        // Bottom left
        neighbours.push(self + UVec2::new(0, 1) - UVec2::new(1, 0));

        // Bottom
        neighbours.push(self + UVec2::new(0, 1));

        // Bottom right
        neighbours.push(self + UVec2::new(1, 1));

        neighbours
    }

    #[inline]
    fn neighbours_checked_no_diag(&self) -> Vec<Self::V> {
        let mut neighbours = Vec::new();

        // Top
        if self.y > 0 {
            neighbours.push(self - UVec2::new(0, 1));
        }

        // Left
        if self.x > 0 {
            neighbours.push(self - UVec2::new(1, 0));
        }

        // Right
        neighbours.push(self + UVec2::new(1, 0));

        // Bottom
        neighbours.push(self + UVec2::new(0, 1));

        neighbours
    }

    #[inline]
    fn neighbours_checked_diag_only(&self) -> Vec<Self::V> {
        let mut neighbours = Vec::new();

        // Top left
        if self.x > 0 && self.y > 0 {
            neighbours.push(self - UVec2::new(1, 1));
        }

        // Top right
        if self.y > 0 {
            neighbours.push(self - UVec2::new(0, 1) + UVec2::new(1, 0));
        }

        // Bottom left
        neighbours.push(self + UVec2::new(0, 1) - UVec2::new(1, 0));

        // Bottom right
        neighbours.push(self + UVec2::new(1, 1));

        neighbours
    }

    #[inline]
    fn neighbours_checked_length(&self, length: u32) -> Vec<Self::V> {
        let mut neighbours = Vec::new();

        // Top left
        if self.x >= length && self.y >= length {
            neighbours.push(self - UVec2::new(1, 1));
        }
        // Top
        if self.y >= length {
            neighbours.push(self - UVec2::new(0, 1));
        }
        // Top right
        if self.y >= length {
            neighbours.push(self - UVec2::new(0, 1) + UVec2::new(1, 0));
        }
        // Left
        if self.x >= length {
            neighbours.push(self - UVec2::new(1, 0));
        }

        // Right
        neighbours.push(self + UVec2::new(1, 0));

        // Bottom left
        neighbours.push(self + UVec2::new(0, 1) - UVec2::new(1, 0));

        // Bottom
        neighbours.push(self + UVec2::new(0, 1));

        // Bottom right
        neighbours.push(self + UVec2::new(1, 1));

        neighbours
    }

    #[inline]
    fn neighbours_checked_no_diag_length(&self, length: u32) -> Vec<Self::V> {
        let mut neighbours = Vec::new();

        // Top
        if self.y >= length {
            neighbours.push(self - UVec2::new(0, 1));
        }

        // Left
        if self.x >= length {
            neighbours.push(self - UVec2::new(1, 0));
        }

        // Right
        neighbours.push(self + UVec2::new(1, 0));

        // Bottom
        neighbours.push(self + UVec2::new(0, 1));

        neighbours
    }

    #[inline]
    fn neighbours_checked_diag_only_length(&self, length: u32) -> Vec<Self::V> {
        let mut neighbours = Vec::new();

        // Top left
        if self.x >= length && self.y >= length {
            neighbours.push(self - UVec2::new(1, 1));
        }

        // Top right
        if self.y >= length {
            neighbours.push(self - UVec2::new(0, 1) + UVec2::new(1, 0));
        }

        // Bottom left
        neighbours.push(self + UVec2::new(0, 1) - UVec2::new(1, 0));

        // Bottom right
        neighbours.push(self + UVec2::new(1, 1));

        neighbours
    }

    #[inline]
    fn neighbours_checked_square(&self, radius: u32) -> Vec<Self::V> {
        let r = radius as i32;
        let square = IVec2::splat(r);

        IVec2::splat(r * 2)
            .positions_inclusive()
            .into_iter()
            .filter_map(|pos_i| {
                if !can_convert_to_uvec(pos_i) {
                    return None;
                }

                let pos = (self.as_ivec2() + (pos_i - square)).as_uvec2();

                if pos == *self {
                    return None;
                }

                Some(pos)
            })
            .collect::<Vec<UVec2>>()
    }

    #[inline]
    fn neighbours_checked_circle(&self, radius: u32) -> Vec<Self::V> {
        let r = radius as i32;
        let square = IVec2::splat(r);

        IVec2::splat(r * 2)
            .positions_inclusive()
            .into_iter()
            .filter_map(|pos_i| {
                if !can_convert_to_uvec(pos_i) {
                    return None;
                }

                let pos = (self.as_ivec2() + (pos_i - square)).as_uvec2();

                let dist = self.as_vec2().distance(pos.as_vec2());

                if dist > radius as f32 {
                    return None;
                }

                if pos == *self {
                    return None;
                }

                Some(pos)
            })
            .collect::<Vec<UVec2>>()
    }
}

#[inline(always)]
fn can_convert_to_uvec(ivec: IVec2) -> bool {
    ivec.min_element() >= 0
}

pub trait Distance {
    type V;

    /// The distance in a straight line
    ///
    /// ### Example:
    /// 1.4 1.0 1.4
    ///
    /// 1.0 pos 1.0
    ///
    /// 1.4 1.0 1.4
    fn distance_euclidian(&self, other: Self::V) -> f32;

    /// Distance with only cardinal movements allowed
    ///
    /// ### Example:
    /// 2 1 2
    ///
    /// 1 p 1
    ///
    /// 2 1 2
    fn distance_manhattan(&self, other: Self::V) -> u32;

    /// Distance where diagonal and cardinal movements both have the same length
    ///
    /// ### Example:
    /// 1 1 1
    ///
    /// 1 p 1
    ///
    /// 1 1 1
    fn distance_chebyshev(&self, other: Self::V) -> u32;
}

impl Distance for IVec2 {
    type V = IVec2;

    #[inline(always)]
    fn distance_euclidian(&self, other: IVec2) -> f32 {
        // Compute difference as f32 and use hypot (fast and stable)
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;

        dx.hypot(dy)
    }

    #[inline(always)]
    fn distance_manhattan(&self, other: IVec2) -> u32 {
        // Use i64 to avoid overflow on abs(i32::MAX)
        let dx = (self.x as i64 - other.x as i64).abs() as u64;
        let dy = (self.y as i64 - other.y as i64).abs() as u64;
        let sum = dx + dy;

        // Clamp to u32::MAX because return type is u32
        if sum > u64::from(u32::MAX) {
            u32::MAX
        } else {
            sum as u32
        }
    }

    #[inline(always)]
    fn distance_chebyshev(&self, other: IVec2) -> u32 {
        let dx = (self.x as i64 - other.x as i64).abs() as u64;
        let dy = (self.y as i64 - other.y as i64).abs() as u64;
        let maxi = if dx > dy { dx } else { dy };

        if maxi > u64::from(u32::MAX) {
            u32::MAX
        } else {
            maxi as u32
        }
    }
}

impl Distance for UVec2 {
    type V = UVec2;

    #[inline(always)]
    fn distance_euclidian(&self, other: UVec2) -> f32 {
        // Convert to f32 and use hypot
        let dx = (self.x as f32) - (other.x as f32);
        let dy = (self.y as f32) - (other.y as f32);

        dx.hypot(dy)
    }

    #[inline(always)]
    fn distance_manhattan(&self, other: UVec2) -> u32 {
        // Compute absolute difference via branching (fast for unsigned)
        let dx = if self.x >= other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let dy = if self.y >= other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        // Sum in u64 then clamp to u32 to avoid overflow
        let sum = dx as u64 + dy as u64;

        if sum > u64::from(u32::MAX) {
            u32::MAX
        } else {
            sum as u32
        }
    }

    #[inline(always)]
    fn distance_chebyshev(&self, other: UVec2) -> u32 {
        let dx = if self.x >= other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let dy = if self.y >= other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        if dx > dy { dx } else { dy }
    }
}

pub trait BoundsCheck {
    type V;

    fn is_in_bounds(&self, bounds: Self::V) -> bool;
}

impl BoundsCheck for UVec2 {
    type V = Self;

    #[inline(always)]
    fn is_in_bounds(&self, bounds: Self::V) -> bool {
        self.x < bounds.x && self.y < bounds.y
    }
}
