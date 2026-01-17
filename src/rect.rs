use crate::*;

#[cfg_attr(feature = "bevy", derive(Reflect))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rect<T: RectScalar> {
    pub top_left: T::V2,
    pub bottom_right: T::V2,
}

impl<T: RectScalar> Rect<T> {
    pub fn from_top_left_and_size(top_left: T::V2, size: T::V2) -> Self {
        Self {
            top_left,
            bottom_right: top_left + size,
        }
    }

    pub fn from_center_and_size(center_pos: T::V2, size: T::V2) -> Self {
        Self {
            top_left: center_pos - (size / T::two()),
            bottom_right: center_pos + (size / T::two()),
        }
    }

    pub fn from_corners(top_left: T::V2, bottom_right: T::V2) -> Self {
        assert!(
            T::v2_x(top_left) < T::v2_x(bottom_right) && T::v2_y(top_left) < T::v2_y(bottom_right),
            "{top_left}, {bottom_right}"
        );

        Self {
            top_left,
            bottom_right,
        }
    }

    #[inline]
    pub fn bounds_from_positions(positions: &[T::V2]) -> Self {
        let mut min_x = T::max();
        let mut min_y = T::max();
        let mut max_x = T::zero();
        let mut max_y = T::zero();

        for &pos in positions {
            if T::v2_x(pos) < min_x {
                min_x = T::v2_x(pos);
            }
            if T::v2_y(pos) < min_y {
                min_y = T::v2_y(pos);
            }
            if T::v2_x(pos) > max_x {
                max_x = T::v2_x(pos);
            }
            if T::v2_y(pos) > max_y {
                max_y = T::v2_y(pos);
            }
        }

        Self::from_corners(T::v2_new(min_x, min_y), T::v2_new(max_x, max_y))
    }

    /// Top left, Top right, Bottom left, Bottom right
    #[inline]
    pub fn corners(&self) -> [T::V2; 4] {
        [
            self.top_left,
            T::v2_new(T::v2_x(self.bottom_right), T::v2_y(self.top_left)),
            T::v2_new(T::v2_x(self.top_left), T::v2_y(self.bottom_right)),
            self.bottom_right,
        ]
    }

    #[inline]
    pub fn center(&self) -> T::V2 {
        self.top_left + (self.size() / T::two())
    }

    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        //! might not need abs()
        (T::v2_min_element(self.size()).to_f32() / T::v2_max_element(self.size()).to_f32()).abs()
    }

    /// Assumes Rect is not square
    #[inline]
    pub fn longest_orientation(&self) -> Orientation {
        debug_assert!(self.width() != self.height());

        if self.width() > self.height() {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        }
    }

    #[inline]
    pub fn width(&self) -> T {
        T::v2_x(self.bottom_right) - T::v2_x(self.top_left)
    }

    #[inline]
    pub fn height(&self) -> T {
        T::v2_y(self.bottom_right) - T::v2_y(self.top_left)
    }

    #[inline]
    pub fn size(&self) -> T::V2 {
        self.bottom_right - self.top_left
    }

    #[inline]
    pub fn area(&self) -> T {
        T::v2_element_product(self.size())
    }

    // #[inline]
    // pub fn smallest_length(&self) -> (u32, Orientation) {
    //     let min = self.size().min_element();

    //     if self.width() > self.height()
    // }

    #[inline]
    pub fn contains_pos(&self, pos: T::V2) -> bool {
        T::v2_x(pos) >= T::v2_x(self.top_left)
            && T::v2_y(pos) >= T::v2_y(self.top_left)
            && T::v2_x(pos) <= T::v2_x(self.bottom_right)
            && T::v2_y(pos) <= T::v2_y(self.bottom_right)
    }
}

impl<T: RectScalar> Rect<T> {
    /// Expands in all directions
    #[must_use]
    #[inline]
    pub fn expanded(&self, amount: T) -> Self {
        Self::from_corners(
            self.top_left - T::v2_new(amount, amount),
            self.bottom_right + T::v2_new(amount, amount),
        )
    }

    /// Expands in all directions
    #[inline]
    pub fn expand(&mut self, amount: T) {
        self.top_left -= T::v2_new(amount, amount);
        self.bottom_right += T::v2_new(amount, amount);
    }

    /// Panics if direction is not cardinal
    #[inline]
    pub fn extend_in_dir(&mut self, direction: Direction, amount: T) {
        match direction {
            Direction::North => *T::v2_y_mut(&mut self.top_left) -= amount,
            Direction::East => *T::v2_x_mut(&mut self.bottom_right) += amount,
            Direction::South => *T::v2_y_mut(&mut self.bottom_right) += amount,
            Direction::West => *T::v2_x_mut(&mut self.top_left) -= amount,
            _ => panic!("Extend direction must be cardinal"),
        }
    }

    /// Shrinks Self in the specified direction eg, North, 2 = BOTTOM edge will move UP by 2
    ///
    /// Panics if direction is not cardinal, or if Rect is too small
    #[inline]
    pub fn shrink_in_dir(&mut self, direction: Direction, amount: T) {
        match direction {
            Direction::North => *T::v2_y_mut(&mut self.bottom_right) -= amount,
            Direction::East => *T::v2_x_mut(&mut self.top_left) += amount,
            Direction::South => *T::v2_y_mut(&mut self.top_left) += amount,
            Direction::West => *T::v2_x_mut(&mut self.bottom_right) -= amount,
            _ => panic!("Shrink direction must be cardinal"),
        }
    }

    /// Returns None if Rect is too small
    pub fn shrink(&self, amount: T) -> Option<Self> {
        // Min Rect size of 2x2 ?
        let min_length = (amount * T::two()) + T::two();

        if self.width() < min_length || self.height() < min_length {
            return None;
        }

        Some(Self::from_corners(
            self.top_left + T::v2_new(amount, amount),
            self.bottom_right - T::v2_new(amount, amount),
        ))
    }

    /// Panics if Rect is too small
    pub fn shrink_in_place(&mut self, amount: T) {
        // Min Rect size of 2x2 ?
        let min_length = (amount * T::two()) + T::two();

        if self.width() < min_length || self.height() < min_length {
            panic!("Rect is too small to shrink by {amount}");
        }

        self.top_left += T::v2_new(amount, amount);
        self.bottom_right -= T::v2_new(amount, amount);
    }

    pub fn scale_up(&mut self, scale_factor: T::V2) {
        self.top_left *= scale_factor;
        self.bottom_right *= scale_factor;
    }

    pub fn scale_down(&mut self, scale_factor: T::V2) {
        self.top_left /= scale_factor;
        self.bottom_right /= scale_factor;
    }

    pub fn bisect_at(&self, orientation: Orientation, split_point: T) -> (Self, Self) {
        let rect_1;
        let rect_2;

        match orientation {
            Orientation::Vertical => {
                let tl_1 = self.top_left;
                let br_1 = T::v2_new(T::v2_x(tl_1) + split_point, T::v2_y(self.bottom_right));

                let tl_2 = T::v2_new(
                    T::v2_x(tl_1) + split_point + T::one(),
                    T::v2_y(self.top_left),
                );
                let br_2 = self.bottom_right;

                rect_1 = Self::from_corners(tl_1, br_1);
                rect_2 = Self::from_corners(tl_2, br_2);
            }
            Orientation::Horizontal => {
                let tl_1 = self.top_left;
                let br_1 = T::v2_new(T::v2_x(self.bottom_right), T::v2_y(tl_1) + split_point);

                let tl_2 = T::v2_new(
                    T::v2_x(self.top_left),
                    T::v2_y(tl_1) + split_point + T::one(),
                );
                let br_2 = self.bottom_right;

                rect_1 = Self::from_corners(tl_1, br_1);
                rect_2 = Self::from_corners(tl_2, br_2);
            }
        }

        (rect_1, rect_2)
    }

    pub fn get_local_coord_at_length_percent(
        &self,
        length_orientation: Orientation,
        length_percent: f32,
    ) -> T {
        match length_orientation {
            Orientation::Vertical => T::from_f32(T::v2_y(self.size()).to_f32() * length_percent),
            Orientation::Horizontal => T::from_f32(T::v2_x(self.size()).to_f32() * length_percent),
        }
    }

    pub fn get_global_coord_at_length_percent(
        &self,
        length_orientation: Orientation,
        length_percent: f32,
    ) -> T {
        match length_orientation {
            Orientation::Vertical => {
                T::v2_x(self.top_left) + T::from_f32(T::v2_y(self.size()).to_f32() * length_percent)
            }
            Orientation::Horizontal => {
                T::v2_y(self.top_left) + T::from_f32(T::v2_x(self.size()).to_f32() * length_percent)
            }
        }
    }

    pub fn bisect_at_length_percent(
        &self,
        orientation: Orientation,
        length_percent: f32,
    ) -> (Self, Self) {
        let rect_1;
        let rect_2;

        match orientation {
            Orientation::Vertical => {
                let split_point = T::from_f32(T::v2_x(self.size()).to_f32() * length_percent);

                let tl_1 = self.top_left;
                let br_1 = T::v2_new(T::v2_x(tl_1) + split_point, T::v2_y(self.bottom_right));

                let tl_2 = T::v2_new(
                    T::v2_x(tl_1) + split_point + T::one(),
                    T::v2_y(self.top_left),
                );
                let br_2 = self.bottom_right;

                rect_1 = Self::from_corners(tl_1, br_1);
                rect_2 = Self::from_corners(tl_2, br_2);
            }
            Orientation::Horizontal => {
                let split_point = T::from_f32(T::v2_y(self.size()).to_f32() * length_percent);

                let tl_1 = self.top_left;
                let br_1 = T::v2_new(T::v2_x(self.bottom_right), T::v2_y(tl_1) + split_point);

                let tl_2 = T::v2_new(
                    T::v2_x(self.top_left),
                    T::v2_y(tl_1) + split_point + T::one(),
                );
                let br_2 = self.bottom_right;

                rect_1 = Self::from_corners(tl_1, br_1);
                rect_2 = Self::from_corners(tl_2, br_2);
            }
        }

        (rect_1, rect_2)
    }
}

// impl<T: RectScalar> PartialOrd for Rect<T> {
//     #[inline]
//     fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for Rect {
//     #[inline]
//     fn cmp(&self, other: &Self) -> core::cmp::Ordering {
//         self.area().cmp(&other.area())
//     }
// }

impl Rect<u32> {
    /// Exclusive
    #[inline]
    pub fn positions(&self) -> Vec<UVec2> {
        let mut positions = Vec::new();

        for y in self.top_left.y..self.bottom_right.y {
            for x in self.top_left.x..self.bottom_right.x {
                positions.push(UVec2::new(x, y));
            }
        }

        positions
    }

    #[inline]
    pub fn positions_inclusive(&self) -> Vec<UVec2> {
        let mut positions = Vec::new();

        for y in self.top_left.y..=self.bottom_right.y {
            for x in self.top_left.x..=self.bottom_right.x {
                positions.push(UVec2::new(x, y));
            }
        }

        positions
    }

    /// TODO - improve performance
    #[inline]
    pub fn border_positions(&self) -> Vec<UVec2> {
        let valid_xs = [self.top_left.x, self.bottom_right.x];
        let valid_ys = [self.top_left.y, self.bottom_right.y];

        self.positions_inclusive()
            .into_iter()
            .filter(|&pos| valid_xs.contains(&pos.x) || valid_ys.contains(&pos.y))
            .collect()
    }
}

impl Rect<i32> {
    /// Exclusive
    #[inline]
    pub fn positions(&self) -> Vec<IVec2> {
        let mut positions = Vec::new();

        for y in self.top_left.y..self.bottom_right.y {
            for x in self.top_left.x..self.bottom_right.x {
                positions.push(IVec2::new(x, y));
            }
        }

        positions
    }

    #[inline]
    pub fn positions_inclusive(&self) -> Vec<IVec2> {
        let mut positions = Vec::new();

        for y in self.top_left.y..=self.bottom_right.y {
            for x in self.top_left.x..=self.bottom_right.x {
                positions.push(IVec2::new(x, y));
            }
        }

        positions
    }

    /// TODO - improve performance
    #[inline]
    pub fn border_positions(&self) -> Vec<IVec2> {
        let valid_xs = [self.top_left.x, self.bottom_right.x];
        let valid_ys = [self.top_left.y, self.bottom_right.y];

        self.positions_inclusive()
            .into_iter()
            .filter(|&pos| valid_xs.contains(&pos.x) || valid_ys.contains(&pos.y))
            .collect()
    }
}
