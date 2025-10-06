use crate::*;

use core::ops::{AddAssign, Div};

impl Matrix<f32> {
    /// Compares two Matrix<f32> values element-wise with tolerance.
    ///
    /// A suitable tolerance is 0.00001
    pub fn approx_eq(&self, other: &Matrix<f32>, tolerance: f32) -> bool {
        if self.size() != other.size() {
            return false;
        }

        for pos in self.size().positions() {
            let value_a = self.get(pos);
            let value_b = other.get(pos);

            if (value_a - value_b).abs() > tolerance {
                return false;
            }
        }

        true
    }
}

impl Matrix<f64> {
    /// Compares two Matrix<f64> values element-wise with tolerance.
    ///
    /// A suitable tolerance is 0.00001
    pub fn approx_eq(&self, other: &Matrix<f64>, tolerance: f64) -> bool {
        if self.size() != other.size() {
            return false;
        }

        for pos in self.size().positions() {
            let value_a = self.get(pos);
            let value_b = other.get(pos);

            if (value_a - value_b).abs() > tolerance {
                return false;
            }
        }

        true
    }
}

impl<T> Matrix<T> {
    #[inline]
    pub fn get_center_pos(&self) -> UVec2 {
        debug_assert!(self.size().x % 2 != 0);

        self.size() / 2
    }

    //TODO? - check if pos in near border, and skip checks if not
    #[inline]
    pub fn valid_dirs(&self, pos: UVec2) -> Vec<Direction> {
        let mut valid_dirs = Vec::with_capacity(4);

        let size = self.size();
        let neighbours = pos.neighbours_no_diag();

        if Self::is_in_bounds_multi(neighbours[0], size) {
            valid_dirs.push(Direction::North);
        }
        if Self::is_in_bounds_multi(neighbours[1], size) {
            valid_dirs.push(Direction::West);
        }
        if Self::is_in_bounds_multi(neighbours[2], size) {
            valid_dirs.push(Direction::East);
        }
        if Self::is_in_bounds_multi(neighbours[3], size) {
            valid_dirs.push(Direction::South);
        }

        valid_dirs
    }

    #[inline]
    pub fn neighbours(&self, pos: UVec2) -> Vec<UVec2> {
        let mut valid = Vec::with_capacity(8);
        let size = self.size();

        let can_go_left = pos.x > 0;
        let can_go_right = pos.x < size.x - 1;
        let can_go_up = pos.y > 0;
        let can_go_down = pos.y < size.y - 1;

        if can_go_up {
            // Top-left (needs left + up)
            if can_go_left {
                valid.push(pos - UVec2::new(1, 1));
            }
            // Top (needs up)
            valid.push(pos - UVec2::new(0, 1));
            // Top-right (needs right + up)
            if can_go_right {
                valid.push((pos + UVec2::new(1, 0)) - UVec2::new(0, 1));
            }
        }
        // Left (needs left)
        if can_go_left {
            valid.push(pos - UVec2::new(1, 0));
        }
        // Right (needs right)
        if can_go_right {
            valid.push(pos + UVec2::new(1, 0));
        }
        if can_go_down {
            // Bottom-left (needs left + down)
            if can_go_left {
                valid.push((pos - UVec2::new(1, 0)) + UVec2::new(0, 1));
            }
            // Bottom (needs down)
            valid.push(pos + UVec2::new(0, 1));
            // Bottom-right (needs right + down)
            if can_go_right {
                valid.push(pos + UVec2::new(1, 1));
            }
        }

        valid
    }

    #[inline]
    pub fn neighbours_no_diag(&self, pos: UVec2) -> Vec<UVec2> {
        let mut valid = Vec::with_capacity(4);
        let size = self.size();

        // Check top (avoid underflow)
        if pos.y > 0 {
            valid.push(pos - UVec2::new(0, 1));
        }
        // Check left (avoid underflow)
        if pos.x > 0 {
            valid.push(pos - UVec2::new(1, 0));
        }
        // Check right (avoid overflow)
        if pos.x < size.x - 1 {
            valid.push(pos + UVec2::new(1, 0));
        }
        // Check bottom (avoid overflow)
        if pos.y < size.y - 1 {
            valid.push(pos + UVec2::new(0, 1));
        }

        valid
    }

    #[inline]
    pub fn neighbours_diag_only(&self, pos: UVec2) -> Vec<UVec2> {
        let mut valid = Vec::with_capacity(8);
        let size = self.size();

        let can_go_left = pos.x > 0;
        let can_go_right = pos.x < size.x - 1;
        let can_go_up = pos.y > 0;
        let can_go_down = pos.y < size.y - 1;

        if can_go_up {
            // Top-left (needs left + up)
            if can_go_left {
                valid.push(pos - UVec2::new(1, 1));
            }
            // Top-right (needs right + up)
            if can_go_right {
                valid.push((pos + UVec2::new(1, 0)) - UVec2::new(0, 1));
            }
        }
        if can_go_down {
            // Bottom-left (needs left + down)
            if can_go_left {
                valid.push((pos - UVec2::new(1, 0)) + UVec2::new(0, 1));
            }
            // Bottom-right (needs right + down)
            if can_go_right {
                valid.push(pos + UVec2::new(1, 1));
            }
        }

        valid
    }

    #[inline]
    pub fn neighbours_radius(&self, pos: UVec2, radius: u32) -> Vec<UVec2> {
        let b_size = IVec2::splat(radius as i32);
        let block = IVec2::splat((radius as i32 * 2) + 1).positions();
        let pos_i = pos.as_ivec2();
        let size = self.size();

        block
            .into_iter()
            .filter_map(|b_pos| {
                let sample_pos = (b_pos - b_size) + pos_i;

                if sample_pos.min_element() < 0 {
                    return None;
                }

                let sample_pos = sample_pos.as_uvec2();

                if sample_pos == pos {
                    return None;
                }

                if !Self::is_in_bounds_multi(sample_pos, size) {
                    return None;
                }

                Some(sample_pos)
            })
            .collect()
    }

    ///TODO - improve
    #[inline]
    pub fn neighbours_wrapping(&self, pos: UVec2) -> Vec<UVec2> {
        let pos_i = pos.as_ivec2();
        let size_i = self.size().as_ivec2();
        let dirs = [
            IVec2::new(0, -1),  // Up
            IVec2::new(0, 1),   // Down
            IVec2::new(-1, 0),  // Left
            IVec2::new(1, 0),   // Right
            IVec2::new(-1, -1), // UpLeft
            IVec2::new(1, -1),  // UpRight
            IVec2::new(-1, 1),  // DownLeft
            IVec2::new(1, 1),   // DownRight
        ];
        let mut neighbours = Vec::new();

        for dir in dirs {
            let new_x = wrap_i32(pos_i.x + dir.x, 0..size_i.max_element());
            let new_y = wrap_i32(pos_i.y + dir.y, 0..size_i.max_element());

            let new = UVec2::new(new_x as u32, new_y as u32);

            neighbours.push(new);
        }

        neighbours
    }

    ///TODO - improve
    #[inline]
    pub fn neighbours_no_diag_wrapping(&self, pos: UVec2) -> Vec<UVec2> {
        let pos_i = pos.as_ivec2();
        let size_i = self.size().as_ivec2();

        let dirs = [
            IVec2::new(0, -1), // Up
            IVec2::new(0, 1),  // Down
            IVec2::new(-1, 0), // Left
            IVec2::new(1, 0),  // Right
        ];
        let mut neighbours = Vec::new();

        for dir in dirs {
            let new_x = wrap_i32(pos_i.x + dir.x, 0..size_i.max_element());
            let new_y = wrap_i32(pos_i.y + dir.y, 0..size_i.max_element());

            let new = UVec2::new(new_x as u32, new_y as u32);

            neighbours.push(new);
        }

        neighbours
    }

    #[inline]
    pub fn neighbours_diag_only_wrapping(&self, pos: UVec2) -> Vec<UVec2> {
        let pos_i = pos.as_ivec2();
        let size_i = self.size().as_ivec2();
        let dirs = [
            IVec2::new(-1, -1), // UpLeft
            IVec2::new(1, -1),  // UpRight
            IVec2::new(-1, 1),  // DownLeft
            IVec2::new(1, 1),   // DownRight
        ];
        let mut neighbours = Vec::new();

        for dir in dirs {
            let new_x = wrap_i32(pos_i.x + dir.x, 0..size_i.max_element());
            let new_y = wrap_i32(pos_i.y + dir.y, 0..size_i.max_element());

            let new = UVec2::new(new_x as u32, new_y as u32);

            neighbours.push(new);
        }

        neighbours
    }

    #[inline]
    pub fn max_value(&self) -> (&T, UVec2)
    where
        T: PartialOrd,
    {
        self.iter_with_pos()
            .max_by(|&(v1, _), &(v2, _)| v1.partial_cmp(v2).unwrap())
            .unwrap()
    }

    #[inline]
    pub fn min_value(&self) -> (&T, UVec2)
    where
        T: PartialOrd,
    {
        self.iter_with_pos()
            .min_by(|&(v1, _), &(v2, _)| v1.partial_cmp(v2).unwrap())
            .unwrap()
    }

    #[inline]
    pub fn mean_value(&self) -> T
    where
        T: Copy + Default + AddAssign + Div<Output = T> + From<u32>,
    {
        let count = T::from(self.element_count());
        let mut sum = T::default();

        for v in self.iter() {
            sum += *v;
        }

        sum / count
    }

    #[inline]
    pub fn count_matches<'a, F>(&'a self, match_fn: F) -> u32
    where
        T: 'a,
        F: Fn(&T, UVec2) -> bool,
    {
        let mut count = 0;

        for (v, pos) in self.iter_with_pos() {
            if match_fn(v, pos) {
                count += 1;
            }
        }

        count
    }

    /// Returns an iterator over all values where filter_fn returns true
    #[inline]
    pub fn extract_values<'a, F>(&'a self, filter_fn: F) -> Vec<&'a T>
    where
        T: 'a,
        F: Fn(&T) -> bool,
    {
        let mut positions = Vec::with_capacity(self.element_count() as usize);

        for v in self.iter() {
            if filter_fn(v) {
                positions.push(v);
            }
        }

        positions
    }

    /// Returns an iterator over all value-position pairs where filter_fn returns true
    #[inline]
    pub fn extract_values_and_positions<'a, F>(&'a self, filter_fn: F) -> Vec<(&'a T, UVec2)>
    where
        T: 'a,
        F: Fn(&T, UVec2) -> bool,
    {
        let mut positions = Vec::with_capacity(self.element_count() as usize);

        for (v, pos) in self.iter_with_pos() {
            if filter_fn(v, pos) {
                positions.push((v, pos));
            }
        }

        positions
    }

    /// Returns an iterator over all positions where filter_fn returns true
    #[inline]
    pub fn extract_positions<'a, F>(&'a self, filter_fn: F) -> Vec<UVec2>
    where
        T: 'a,
        F: Fn(&T, UVec2) -> bool,
    {
        let mut positions = Vec::with_capacity(self.element_count() as usize);

        for (v, pos) in self.iter_with_pos() {
            if filter_fn(v, pos) {
                positions.push(pos);
            }
        }

        positions
    }
}
