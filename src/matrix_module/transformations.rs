use crate::*;

use core::ops::{AddAssign, Div};

impl Matrix<f32> {
    #[inline]
    pub fn normalise(&mut self) {
        let max_value = *self.max_value().0;

        for v in self.iter_mut() {
            *v /= max_value;
        }
    }
}

impl Matrix<f64> {
    #[inline]
    pub fn normalise(&mut self) {
        let max_value = *self.max_value().0;

        for v in self.iter_mut() {
            *v /= max_value;
        }
    }
}

impl<T: Clone + Debug> Matrix<T> {
    #[must_use]
    #[inline]
    pub fn map<R: Clone + Debug>(&self, map_fn: impl Fn(&T) -> R) -> Matrix<R> {
        let mut mapped = Vec::with_capacity(self.element_count() as usize);

        for t in self.iter() {
            mapped.push(map_fn(t));
        }

        Matrix::from_elements(mapped, self.size())
    }

    #[must_use]
    #[inline]
    pub fn map_with_pos<R: Clone + Debug>(
        &self,
        mapping_function: impl Fn(&T, UVec2) -> R,
    ) -> Matrix<R> {
        let mut mapped = Vec::with_capacity(self.element_count() as usize);

        for (t, pos) in self.iter_with_pos() {
            mapped.push(mapping_function(t, pos));
        }

        Matrix::from_elements(mapped, self.size())
    }

    #[inline]
    pub fn smooth(&mut self, kernal_size: u32, with_diagonals: bool)
    where
        T: Copy + AddAssign + Div + Into<f32> + From<f32>,
    {
        let kernal_positions: Vec<IVec2> = IVec2::splat(kernal_size as i32 * 2)
            .positions_inclusive()
            .into_iter()
            .map(|pos| pos - IVec2::splat(kernal_size as i32))
            .collect();
        let mut new = self.clone();

        for (t, pos) in self.iter_with_pos() {
            let mut total = <T as Into<f32>>::into(*t);
            let mut count = 0.0;

            for kernal_pos in &kernal_positions {
                let p = (pos.as_ivec2() + kernal_pos).as_uvec2();

                if !self.is_in_bounds(p) {
                    continue;
                }

                total += <T as Into<f32>>::into(*self.get(p)); //(*self.get(neighbour)).into();
                count += 1.0;
            }

            *new.get_mut(pos) = (total / count).into();
        }

        *self = new;
    }

    // #[must_use]
    // #[inline]
    // pub fn stretch_and_fill(&self, spacing: u32, fill_value: T) -> Matrix<T>
    // where
    //     T: Copy,
    // {
    //     assert!(spacing != 0);

    //     let mut new_inner = Vec::new();

    //     let new_size_x = (self.size().x * (spacing + 1)) - spacing;
    //     let new_size_y = (self.size().y * (spacing + 1)) - spacing;

    //     for (row_idx, row) in self.get_inner().iter().enumerate() {
    //         let mut new_row = Vec::new();

    //         for (value_idx, value) in row.iter().enumerate() {
    //             new_row.push(*value);

    //             if (value_idx as u32) < self.size().x - 1 {
    //                 for _ in 0..spacing {
    //                     new_row.push(fill_value);
    //                 }
    //             }
    //         }

    //         new_inner.push(new_row);

    //         if (row_idx as u32) < self.size().y - 1 {
    //             for _ in 0..spacing {
    //                 new_inner.push(vec![fill_value; new_size_x as usize - 1]);
    //             }
    //         }
    //     }

    //     Matrix::from_inner(new_inner)
    // }

    // /// Returns a copy of the matrix, rotated in the specified direction
    // #[inline]
    // #[must_use]
    // pub fn rotate(&self, direction: Rotation) -> Matrix<T> {
    //     let rows = self.get_inner().len();
    //     if rows == 0 {
    //         return Matrix::from_inner(Vec::new());
    //     }
    //     let cols = self.get_inner()[0].len();

    //     let mut result = vec![vec![self.get_inner()[0][0].clone(); rows]; cols];

    //     match direction {
    //         Rotation::D90 => {
    //             for (i, row) in self.get_inner().iter().enumerate() {
    //                 for (j, val) in row.iter().enumerate() {
    //                     result[j][rows - 1 - i] = val.clone();
    //                 }
    //             }
    //         }
    //         Rotation::D180 => {
    //             for (i, row) in self.get_inner().iter().enumerate() {
    //                 for (j, val) in row.iter().enumerate() {
    //                     result[cols - 1 - j][rows - 1 - i] = val.clone();
    //                 }
    //             }
    //         }
    //         Rotation::D270 => {
    //             for (i, row) in self.get_inner().iter().enumerate() {
    //                 for (j, val) in row.iter().enumerate() {
    //                     result[cols - 1 - j][i] = val.clone();
    //                 }
    //             }
    //         }
    //     }

    //     Matrix::from_inner(result)
    // }

    // /// Returns a copy of the matrix, flipped in the specified direction
    // #[inline]
    // #[must_use]
    // pub fn flip(&self, direction: Flip) -> Matrix<T> {
    //     match direction {
    //         Flip::Horizontal => Matrix::from_inner(
    //             self.get_inner()
    //                 .iter()
    //                 .map(|row| row.iter().rev().cloned().collect())
    //                 .collect(),
    //         ),
    //         Flip::Vertical => Matrix::from_inner(self.get_inner().iter().rev().cloned().collect()),
    //     }
    // }
}

/// Clockwise rotations in 90 degree increments
#[derive(Clone, Copy)]
pub enum MatrixRotation {
    /// 90 degrees clockwise
    D90,
    /// 180 degrees clockwise
    D180,
    /// 270 degrees clockwise
    D270,
}

#[derive(Clone, Copy)]
pub enum Flip {
    Horizontal,
    Vertical,
}
