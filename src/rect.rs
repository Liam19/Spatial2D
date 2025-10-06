use std::collections::BinaryHeap;

use crate::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rect {
    pub top_left_corner: UVec2,
    pub bottom_right_corner: UVec2,
}

impl Rect {
    pub fn from_top_left_and_size(top_left_corner: UVec2, size: UVec2) -> Self {
        Self {
            top_left_corner,
            bottom_right_corner: top_left_corner + size,
        }
    }

    pub fn from_center_and_size(center_pos: UVec2, size: UVec2) -> Self {
        Self {
            top_left_corner: center_pos - (size / 2),
            bottom_right_corner: center_pos + (size / 2),
        }
    }

    pub fn from_corners(top_left_corner: UVec2, bottom_right_corner: UVec2) -> Self {
        assert!(
            top_left_corner.x < bottom_right_corner.x && top_left_corner.y < bottom_right_corner.y,
            "{top_left_corner}, {bottom_right_corner}"
        );

        Self {
            top_left_corner,
            bottom_right_corner,
        }
    }

    #[inline]
    pub fn bounds_from_positions(positions: &[UVec2]) -> Self {
        let mut min_x = u32::MAX;
        let mut min_y = u32::MAX;
        let mut max_x = 0u32;
        let mut max_y = 0u32;

        for &pos in positions {
            if pos.x < min_x {
                min_x = pos.x;
            }
            if pos.y < min_y {
                min_y = pos.y;
            }
            if pos.x > max_x {
                max_x = pos.x;
            }
            if pos.y > max_y {
                max_y = pos.y;
            }
        }

        Self::from_corners(UVec2::new(min_x, min_y), UVec2::new(max_x, max_y))
    }

    /// Top left, Top right, Bottom left, Bottom right
    #[inline]
    pub fn corners(&self) -> [UVec2; 4] {
        [
            self.top_left_corner,
            UVec2::new(self.bottom_right_corner.x, self.top_left_corner.y),
            UVec2::new(self.top_left_corner.x, self.bottom_right_corner.y),
            self.bottom_right_corner,
        ]
    }

    #[inline]
    pub fn center(&self) -> UVec2 {
        self.top_left_corner + (self.size() / 2)
    }

    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        //! might not need abs()
        (self.size().min_element() as f32 / self.size().max_element() as f32).abs()
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
    pub fn width(&self) -> u32 {
        self.bottom_right_corner.x - self.top_left_corner.x
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.bottom_right_corner.y - self.top_left_corner.y
    }

    #[inline]
    pub fn size(&self) -> UVec2 {
        self.bottom_right_corner - self.top_left_corner
    }

    #[inline]
    pub fn area(&self) -> u32 {
        self.size().element_product()
    }

    // #[inline]
    // pub fn smallest_length(&self) -> (u32, Orientation) {
    //     let min = self.size().min_element();

    //     if self.width() > self.height()
    // }

    /// Inclusive
    #[inline]
    pub fn positions(&self) -> Vec<UVec2> {
        let mut positions = Vec::new();

        for y in self.top_left_corner.y..=self.bottom_right_corner.y {
            for x in self.top_left_corner.x..=self.bottom_right_corner.x {
                positions.push(UVec2::new(x, y));
            }
        }

        positions
    }

    /// TODO - improve performance
    #[inline]
    pub fn border_positions(&self) -> Vec<UVec2> {
        let valid_xs = [self.top_left_corner.x, self.bottom_right_corner.x];
        let valid_ys = [self.top_left_corner.y, self.bottom_right_corner.y];

        self.positions()
            .into_iter()
            .filter(|&pos| valid_xs.contains(&pos.x) || valid_ys.contains(&pos.y))
            .collect()
    }

    #[inline]
    pub fn contains_pos(&self, pos: UVec2) -> bool {
        pos.x >= self.top_left_corner.x
            && pos.y >= self.top_left_corner.y
            && pos.x <= self.bottom_right_corner.x
            && pos.y <= self.bottom_right_corner.y
    }
}

impl Rect {
    /// Expands in all directions
    #[must_use]
    #[inline]
    pub fn expand(&self, amount: u32) -> Self {
        Self::from_corners(
            self.top_left_corner - UVec2::new(amount, amount),
            self.bottom_right_corner + UVec2::new(amount, amount),
        )
    }

    /// Expands in all directions
    #[inline]
    pub fn expand_in_place(&mut self, amount: u32) {
        self.top_left_corner -= UVec2::new(amount, amount);
        self.bottom_right_corner += UVec2::new(amount, amount);
    }

    /// Panics if direction is not cardinal
    #[inline]
    pub fn extend_in_dir(&mut self, direction: Direction, amount: u32) {
        match direction {
            Direction::North => self.top_left_corner.y -= amount,
            Direction::East => self.bottom_right_corner.x += amount,
            Direction::South => self.bottom_right_corner.y += amount,
            Direction::West => self.top_left_corner.x -= amount,
            _ => panic!("Extend direction must be cardinal"),
        }
    }

    /// Shrinks Self in the specified direction eg, North, 2 = BOTTOM edge will move UP by 2
    ///
    /// Panics if direction is not cardinal, or if Rect is too small
    #[inline]
    pub fn shrink_in_dir(&mut self, direction: Direction, amount: u32) {
        match direction {
            Direction::North => self.bottom_right_corner.y -= amount,
            Direction::East => self.top_left_corner.x += amount,
            Direction::South => self.top_left_corner.y += amount,
            Direction::West => self.bottom_right_corner.x -= amount,
            _ => panic!("Shrink direction must be cardinal"),
        }
    }

    /// Returns None if Rect is too small
    pub fn shrink(&self, amount: u32) -> Option<Self> {
        // Min Rect size of 2x2 ?
        let min_length = (amount * 2) + 2;

        if self.width() < min_length || self.height() < min_length {
            return None;
        }

        Some(Self::from_corners(
            self.top_left_corner + UVec2::new(amount, amount),
            self.bottom_right_corner - UVec2::new(amount, amount),
        ))
    }

    /// Panics if Rect is too small
    pub fn shrink_in_place(&mut self, amount: u32) {
        // Min Rect size of 2x2 ?
        let min_length = (amount * 2) + 2;

        if self.width() < min_length || self.height() < min_length {
            panic!("Rect is too small to shrink by {amount}");
        }

        self.top_left_corner += UVec2::new(amount, amount);
        self.bottom_right_corner -= UVec2::new(amount, amount);
    }

    #[deprecated = "Need to fix ?"]
    /// Must be power of 2
    pub fn scale_up(&mut self, scale_factor: u32) {
        debug_assert!(scale_factor.is_power_of_two());

        let scale = UVec2::splat(scale_factor);

        self.top_left_corner *= scale;
        self.bottom_right_corner *= scale;
    }

    #[deprecated = "Need to fix ?"]
    pub fn scale_down(&mut self, scale_factor: u32) {
        let scale = UVec2::splat(scale_factor);

        self.top_left_corner /= scale;
        self.bottom_right_corner /= scale;
    }

    #[inline]
    pub fn scale_vector(&mut self, scale_vector: Vec2) {
        self.top_left_corner = (self.top_left_corner.as_vec2() * scale_vector).as_uvec2();
        self.bottom_right_corner = (self.bottom_right_corner.as_vec2() * scale_vector).as_uvec2();
    }

    pub fn bisect_at(&self, orientation: Orientation, split_point: u32) -> (Self, Self) {
        let rect_1;
        let rect_2;

        match orientation {
            Orientation::Vertical => {
                let tl_1 = self.top_left_corner;
                let br_1 = UVec2::new(tl_1.x + split_point, self.bottom_right_corner.y);

                let tl_2 = UVec2::new(tl_1.x + split_point + 1, self.top_left_corner.y);
                let br_2 = self.bottom_right_corner;

                rect_1 = Self::from_corners(tl_1, br_1);
                rect_2 = Self::from_corners(tl_2, br_2);
            }
            Orientation::Horizontal => {
                let tl_1 = self.top_left_corner;
                let br_1 = UVec2::new(self.bottom_right_corner.x, tl_1.y + split_point);

                let tl_2 = UVec2::new(self.top_left_corner.x, tl_1.y + split_point + 1);
                let br_2 = self.bottom_right_corner;

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
    ) -> u32 {
        match length_orientation {
            Orientation::Vertical => (self.size().y as f32 * length_percent) as u32,
            Orientation::Horizontal => (self.size().x as f32 * length_percent) as u32,
        }
    }

    pub fn get_global_coord_at_length_percent(
        &self,
        length_orientation: Orientation,
        length_percent: f32,
    ) -> u32 {
        match length_orientation {
            Orientation::Vertical => {
                self.top_left_corner.y + (self.size().y as f32 * length_percent) as u32
            }
            Orientation::Horizontal => {
                self.top_left_corner.x + (self.size().x as f32 * length_percent) as u32
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
                let split_point = (self.size().x as f32 * length_percent) as u32;

                let tl_1 = self.top_left_corner;
                let br_1 = UVec2::new(tl_1.x + split_point, self.bottom_right_corner.y);

                let tl_2 = UVec2::new(tl_1.x + split_point + 1, self.top_left_corner.y);
                let br_2 = self.bottom_right_corner;

                rect_1 = Self::from_corners(tl_1, br_1);
                rect_2 = Self::from_corners(tl_2, br_2);
            }
            Orientation::Horizontal => {
                let split_point = (self.size().y as f32 * length_percent) as u32;

                let tl_1 = self.top_left_corner;
                let br_1 = UVec2::new(self.bottom_right_corner.x, tl_1.y + split_point);

                let tl_2 = UVec2::new(self.top_left_corner.x, tl_1.y + split_point + 1);
                let br_2 = self.bottom_right_corner;

                rect_1 = Self::from_corners(tl_1, br_1);
                rect_2 = Self::from_corners(tl_2, br_2);
            }
        }

        (rect_1, rect_2)
    }
}

impl PartialOrd for Rect {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rect {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.area().cmp(&other.area())
    }
}
