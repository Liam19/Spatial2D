//! MIGHT NOT WORK FOR NON-SQUARE MATRICES

use crate::*;

pub trait MatrixScale {
    #[must_use]
    fn scale_up(&self, factor: u32) -> Self;

    #[must_use]
    fn scale_down(&self, factor: u32) -> Self;
}

impl MatrixScale for Matrix<u32> {
    fn scale_up(&self, factor: u32) -> Self {
        assert!(factor.is_power_of_two());

        let base_size = self.size();
        let scaled_size = base_size * factor;

        let mut new = Matrix::splat(scaled_size, 0);

        for pos in base_size.positions() {
            let value = *self.get(pos);
            let scaled_pos = pos * factor;

            let scaled_positions = UVec2::splat(factor)
                .positions()
                .into_iter()
                .map(|block_pos| block_pos + scaled_pos);

            for scaled_pos in scaled_positions {
                new.set(scaled_pos, value);
            }
        }

        new
    }

    fn scale_down(&self, factor: u32) -> Self {
        assert!(factor.is_power_of_two());
        assert!(
            self.size().x % factor == 0 && self.size().y % factor == 0,
            "Matrix dimensions must be divisible by scale factor"
        );

        let base_size = self.size();
        let scaled_size = base_size / factor;

        // Take the "center" value of each block (closest to center as possible, with bias to top-left)
        let block_size = UVec2::splat(factor);
        let center = (block_size.as_vec2() / 2.0).as_uvec2() - UVec2::ONE;

        let mut new = Matrix::splat(scaled_size, 0);

        for scaled_pos in scaled_size.positions() {
            let sample_pos = (scaled_pos * factor) + center;

            let value = *self.get(sample_pos);

            new.set(scaled_pos, value);
        }

        new
    }
}

impl MatrixScale for Matrix<bool> {
    fn scale_up(&self, factor: u32) -> Self {
        assert!(factor.is_power_of_two());

        let base_size = self.size();
        let scaled_size = base_size * factor;

        let mut new = Matrix::splat(scaled_size, false);

        for pos in base_size.positions() {
            let value = *self.get(pos);
            let scaled_pos = pos * factor;

            let scaled_positions = UVec2::splat(factor)
                .positions()
                .into_iter()
                .map(|block_pos| block_pos + scaled_pos);

            for scaled_pos in scaled_positions {
                new.set(scaled_pos, value);
            }
        }

        new
    }

    fn scale_down(&self, factor: u32) -> Self {
        assert!(factor.is_power_of_two());
        assert!(
            self.size().x % factor == 0 && self.size().y % factor == 0,
            "Matrix dimensions must be divisible by scale factor"
        );

        let base_size = self.size();
        let scaled_size = base_size / factor;

        // Take the "center" value of each block (closest to center as possible, with bias to top-left)
        let block_size = UVec2::splat(factor);
        let center = (block_size.as_vec2() / 2.0).as_uvec2() - UVec2::ONE;

        let mut new = Matrix::splat(scaled_size, false);

        for scaled_pos in scaled_size.positions() {
            let sample_pos = (scaled_pos * factor) + center;

            let value = *self.get(sample_pos);

            new.set(scaled_pos, value);
        }

        new
    }
}

impl MatrixScale for Matrix<f32> {
    /// Uses bilinear interpolation.
    fn scale_up(&self, factor: u32) -> Self {
        let base_size = self.size();
        let scaled_size = base_size * factor;

        let mut output = Matrix::splat(scaled_size, 0.0);

        // Compute scaling factors
        let scale_x = (base_size.x - 1) as f32 / (scaled_size.x - 1).max(1) as f32;
        let scale_y = (base_size.y - 1) as f32 / (scaled_size.y - 1).max(1) as f32;

        for y in 0..scaled_size.y {
            for x in 0..scaled_size.x {
                // Map output pixel to position in input space
                let src_pos = Vec2::new(x as f32 * scale_x, y as f32 * scale_y);
                let value = self.bilinear_interpolate(src_pos);

                output.set(UVec2::new(x, y), value);
            }
        }

        output
    }

    /// Uses position averaging
    fn scale_down(&self, factor: u32) -> Self {
        assert!(factor.is_power_of_two());
        assert!(
            self.size().x % factor == 0 && self.size().y % factor == 0,
            "Matrix dimensions must be divisible by scale factor"
        );

        let base_size = self.size();
        let scaled_size = base_size / factor;
        let block = UVec2::splat(factor);

        let mut new = Matrix::splat(base_size / factor, 0.0);

        for pos in scaled_size.positions() {
            let base_pos = pos * factor;

            let mut sum = 0.0;
            let mut count = 0;

            for block_pos in block.positions() {
                count += 1;
                sum += self.get(block_pos + base_pos);
            }

            let average = sum / count as f32;

            new.set(pos, average);
        }

        new
    }
}

// /// Helper function for bilinear interpolation
// fn bilinear_interpolate(
//     pos: Vec2,
//     top_left: f32,
//     top_right: f32,
//     bottom_left: f32,
//     bottom_right: f32,
// ) -> f32 {
//     let top = top_left * (1.0 - pos.x) + top_right * pos.x;
//     let bottom = bottom_left * (1.0 - pos.x) + bottom_right * pos.x;
//     top * (1.0 - pos.y) + bottom * pos.y
// }

impl Matrix<f32> {
    /// Performs bilinear interpolation on a Matrix<f32> at a fractional position.
    ///
    /// Returns an interpolated value between the four nearest neighbors.
    pub fn bilinear_interpolate(&self, pos: Vec2) -> f32 {
        let size = self.size(); // Assume this returns glam::UVec2 (width, height)

        // Check if position is within valid interpolation bounds
        // (must leave space for +1 in both x and y)
        if pos.x < 0.0
            || pos.y < 0.0
            || pos.x >= size.x as f32 - 1.0
            || pos.y >= size.y as f32 - 1.0
        {
            return 0.0; // Out-of-bounds positions return 0.0 (you could handle this differently)
        }

        // Integer coordinates of the top-left corner of the 2x2 interpolation cell
        let x0 = pos.x.floor() as u32;
        let y0 = pos.y.floor() as u32;

        // Integer coordinates of the bottom-right corner
        let x1 = x0 + 1;
        let y1 = y0 + 1;

        // Fractional parts within the cell (how far we are from x0/y0)
        let tx = pos.x - x0 as f32;
        let ty = pos.y - y0 as f32;

        // Sample the four surrounding cell values
        let v00 = self.get(UVec2::new(x0, y0)); // top-left
        let v10 = self.get(UVec2::new(x1, y0)); // top-right
        let v01 = self.get(UVec2::new(x0, y1)); // bottom-left
        let v11 = self.get(UVec2::new(x1, y1)); // bottom-right

        // Interpolate horizontally between top two and bottom two
        let top = v00 * (1.0 - tx) + v10 * tx;
        let bottom = v01 * (1.0 - tx) + v11 * tx;

        // Interpolate vertically between the top and bottom rows
        top.mul_add(1.0 - ty, bottom * ty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_up_u32_factor_2() {
        // 2x2 matrix
        let original = Matrix::from_elements_2d(vec![vec![1, 2], vec![3, 4]]);

        let scaled = original.scale_up(2);

        // Expected 4x4 matrix
        let expected = Matrix::from_elements_2d(vec![
            vec![1, 1, 2, 2],
            vec![1, 1, 2, 2],
            vec![3, 3, 4, 4],
            vec![3, 3, 4, 4],
        ]);

        assert_eq!(scaled, expected);
    }

    #[test]
    fn test_scale_up_u32_factor_4() {
        // 2x2 matrix
        let original = Matrix::from_elements_2d(vec![vec![1, 2], vec![3, 4]]);

        let scaled = original.scale_up(4);

        // Expected 4x4 matrix
        let expected = Matrix::from_elements_2d(vec![
            vec![1, 1, 1, 1, 2, 2, 2, 2],
            vec![1, 1, 1, 1, 2, 2, 2, 2],
            vec![1, 1, 1, 1, 2, 2, 2, 2],
            vec![1, 1, 1, 1, 2, 2, 2, 2],
            vec![3, 3, 3, 3, 4, 4, 4, 4],
            vec![3, 3, 3, 3, 4, 4, 4, 4],
            vec![3, 3, 3, 3, 4, 4, 4, 4],
            vec![3, 3, 3, 3, 4, 4, 4, 4],
        ]);

        assert_eq!(scaled, expected);
    }

    #[test]
    fn test_scale_down_u32_factor_2() {
        // 4x4 matrix
        let original = Matrix::from_elements_2d(vec![
            vec![1, 1, 2, 2],
            vec![1, 1, 2, 2],
            vec![3, 3, 4, 4],
            vec![3, 3, 4, 4],
        ]);

        let scaled = original.scale_down(2);

        // Expected 2x2 matrix
        let expected = Matrix::from_elements_2d(vec![vec![1, 2], vec![3, 4]]);

        assert_eq!(scaled, expected);
    }

    #[test]
    fn test_scale_down_u32_factor_4() {
        // 8x8 matrix
        let original = Matrix::from_elements_2d(vec![
            vec![1, 1, 1, 1, 2, 2, 2, 2],
            vec![1, 1, 1, 1, 2, 2, 2, 2],
            vec![1, 1, 1, 1, 2, 2, 2, 2],
            vec![1, 1, 1, 1, 2, 2, 2, 2],
            vec![3, 3, 3, 3, 4, 4, 4, 4],
            vec![3, 3, 3, 3, 4, 4, 4, 4],
            vec![3, 3, 3, 3, 4, 4, 4, 4],
            vec![3, 3, 3, 3, 4, 4, 4, 4],
        ]);

        let scaled = original.scale_down(4);

        // Expected 2x2 matrix
        let expected = Matrix::from_elements_2d(vec![vec![1, 2], vec![3, 4]]);

        assert_eq!(scaled, expected);
    }

    #[test]
    #[should_panic(expected = "Matrix dimensions must be divisible by scale factor")]
    fn test_scale_down_u32_invalid_dimensions() {
        // 3x3 matrix (not divisible by 2)
        let original = Matrix::from_elements_2d(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        let _ = original.scale_down(2);
    }

    #[test]
    fn test_scale_up_down_u32_roundtrip() {
        // Original matrix
        let original = Matrix::from_elements_2d(vec![vec![5, 10], vec![15, 20]]);

        // Scale up then down
        let scaled_up = original.scale_up(2);
        let scaled_down = scaled_up.scale_down(2);

        // Should get back original matrix
        assert_eq!(original, scaled_down);
    }

    const Y: bool = true;
    const N: bool = false;

    #[test]
    fn test_scale_up_bool_factor_2() {
        // 2x2 matrix
        let original = Matrix::from_elements_2d(vec![vec![N, Y], vec![Y, N]]);

        let scaled = original.scale_up(2);

        // Expected 4x4 matrix
        let expected = Matrix::from_elements_2d(vec![
            vec![N, N, Y, Y],
            vec![N, N, Y, Y],
            vec![Y, Y, N, N],
            vec![Y, Y, N, N],
        ]);

        assert_eq!(scaled, expected);
    }

    #[test]
    fn test_scale_up_bool_factor_4() {
        // 2x2 matrix
        let original = Matrix::from_elements_2d(vec![vec![N, Y], vec![Y, N]]);

        let scaled = original.scale_up(4);

        // Expected 4x4 matrix
        let expected = Matrix::from_elements_2d(vec![
            vec![N, N, N, N, Y, Y, Y, Y],
            vec![N, N, N, N, Y, Y, Y, Y],
            vec![N, N, N, N, Y, Y, Y, Y],
            vec![N, N, N, N, Y, Y, Y, Y],
            vec![Y, Y, Y, Y, N, N, N, N],
            vec![Y, Y, Y, Y, N, N, N, N],
            vec![Y, Y, Y, Y, N, N, N, N],
            vec![Y, Y, Y, Y, N, N, N, N],
        ]);

        assert_eq!(scaled, expected);
    }

    #[test]
    fn test_scale_down_bool_factor_2() {
        // 4x4 matrix
        let original = Matrix::from_elements_2d(vec![
            vec![N, N, Y, Y],
            vec![N, N, Y, Y],
            vec![Y, Y, N, N],
            vec![Y, Y, N, N],
        ]);

        let scaled = original.scale_down(2);

        // Expected 2x2 matrix
        let expected = Matrix::from_elements_2d(vec![vec![N, Y], vec![Y, N]]);

        assert_eq!(scaled, expected);
    }

    #[test]
    fn test_scale_down_bool_factor_4() {
        // 8x8 matrix
        let original = Matrix::from_elements_2d(vec![
            vec![N, N, N, N, Y, Y, Y, Y],
            vec![N, N, N, N, Y, Y, Y, Y],
            vec![N, N, N, N, Y, Y, Y, Y],
            vec![N, N, N, N, Y, Y, Y, Y],
            vec![Y, Y, Y, Y, N, N, N, N],
            vec![Y, Y, Y, Y, N, N, N, N],
            vec![Y, Y, Y, Y, N, N, N, N],
            vec![Y, Y, Y, Y, N, N, N, N],
        ]);

        let scaled = original.scale_down(4);

        // Expected 2x2 matrix
        let expected = Matrix::from_elements_2d(vec![vec![N, Y], vec![Y, N]]);

        assert_eq!(scaled, expected);
    }

    #[test]
    #[should_panic(expected = "Matrix dimensions must be divisible by scale factor")]
    fn test_scale_down_bool_invalid_dimensions() {
        // 3x3 matrix (not divisible by 2)
        let original = Matrix::from_elements_2d(vec![vec![N, Y, N], vec![Y, N, Y], vec![N, Y, N]]);

        let _ = original.scale_down(2);
    }

    #[test]
    fn test_scale_up_down_bool_roundtrip() {
        // Original matrix
        let original = Matrix::from_elements_2d(vec![vec![Y, N], vec![N, Y]]);

        // Scale up then down
        let scaled_up = original.scale_up(2);
        let scaled_down = scaled_up.scale_down(2);

        // Should get back original matrix
        assert_eq!(original, scaled_down);
    }

    //  Original
    // [
    //     [1.0, 2.0],
    //     [3.0, 4.0],
    // ]

    // Scaled
    // [[1.0, 1.3333333, 1.6666667, 0.0],
    //  [1.6666666, 2.0, 2.3333335, 0.0],
    //  [2.3333333, 2.6666667, 3.0000002, 0.0],
    //  [0.0, 0.0, 0.0, 0.0]]

    // Expected
    // [[1.0, 1.0, 2.0, 2.0],
    //  [1.0, 1.0, 2.0, 2.0],
    //  [3.0, 3.0, 4.0, 4.0],
    //  [3.0, 3.0, 4.0, 4.0]]

    #[test]
    fn test_scale_up_f32_factor_2() {
        // 2x2 matrix
        let original = Matrix::from_elements_2d(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let scaled = original.scale_up(2);

        // Expected 4x4 matrix
        let expected = Matrix::from_elements_2d(vec![
            vec![1.0, 1.0, 2.0, 2.0],
            vec![1.0, 1.0, 2.0, 2.0],
            vec![3.0, 3.0, 4.0, 4.0],
            vec![3.0, 3.0, 4.0, 4.0],
        ]);

        assert!(
            scaled.approx_eq(&expected, 0.00001),
            "Scaled: {:?}, Expected {:?}",
            scaled,
            expected
        );
    }

    // #[test]
    // fn test_scale_up_f32_factor_4() {
    //     // 2x2 matrix
    //     let original = Matrix::from_elements_2d(vec![vec![1, 2], vec![3, 4]]);

    //     let scaled = original.scale_up(4);

    //     // Expected 4x4 matrix
    //     let expected = Matrix::from_elements_2d(vec![
    //         vec![1, 1, 1, 1, 2, 2, 2, 2],
    //         vec![1, 1, 1, 1, 2, 2, 2, 2],
    //         vec![1, 1, 1, 1, 2, 2, 2, 2],
    //         vec![1, 1, 1, 1, 2, 2, 2, 2],
    //         vec![3, 3, 3, 3, 4, 4, 4, 4],
    //         vec![3, 3, 3, 3, 4, 4, 4, 4],
    //         vec![3, 3, 3, 3, 4, 4, 4, 4],
    //         vec![3, 3, 3, 3, 4, 4, 4, 4],
    //     ]);

    //     assert_eq!(scaled, expected);
    // }

    #[test]
    fn test_scale_down_f32_factor_2() {
        // 4x4 matrix
        let original = Matrix::from_elements_2d(vec![
            vec![1.0, 1.0, 2.0, 2.0],
            vec![1.0, 1.0, 2.0, 2.0],
            vec![3.0, 3.0, 4.0, 4.0],
            vec![3.0, 3.0, 4.0, 4.0],
        ]);

        let scaled = original.scale_down(2);

        // Expected 2x2 matrix
        let expected = Matrix::from_elements_2d(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        assert!(
            scaled.approx_eq(&expected, 0.00001),
            "Scaled: {:?}, Expected {:?}",
            scaled,
            expected
        );
        // assert_eq!(scaled, expected);
    }

    #[test]
    fn test_scale_down_f32_factor_4() {
        // 8x8 matrix
        let original = Matrix::from_elements_2d(vec![
            vec![1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            vec![1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            vec![1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            vec![1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            vec![3.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0],
            vec![3.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0],
            vec![3.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0],
            vec![3.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0],
        ]);

        let scaled = original.scale_down(4);

        // Expected 2x2 matrix
        let expected = Matrix::from_elements_2d(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        assert!(
            scaled.approx_eq(&expected, 0.00001),
            "Scaled: {:?}, Expected {:?}",
            scaled,
            expected
        );
    }

    #[test]
    #[should_panic(expected = "Matrix dimensions must be divisible by scale factor")]
    fn test_scale_down_f32_invalid_dimensions() {
        // 3x3 matrix (not divisible by 2)
        let original = Matrix::from_elements_2d(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ]);

        let _ = original.scale_down(2);
    }

    // #[test]
    // fn test_scale_up_down_f32_roundtrip() {
    //     // Original matrix
    //     let original = Matrix::from_elements_2d(vec![vec![5, 10], vec![15, 20]]);

    //     // Scale up then down
    //     let scaled_up = original.scale_up(2);
    //     let scaled_down = scaled_up.scale_down(2);

    //     // Should get back original matrix
    //     assert_eq!(original, scaled_down);
    // }
}
