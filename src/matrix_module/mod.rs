mod algorithms;
mod analysis;
mod iterators;
mod matrix;
mod scaling;
mod transformations;

pub use algorithms::*;
pub use analysis::*;
pub use iterators::*;
pub use matrix::*;
pub use scaling::*;
pub use transformations::*;

// #[test]
// fn test_neighbours() {
//     let mut matrix = FlatMatrix::<u32>::splat(UVec2::splat(8192), 420);

//     let edge_pos = UVec2::splat(8191);

//     let neighbours = matrix.neighbours(edge_pos);
//     let neighbours_diag = matrix.neighbours_with_diag(edge_pos);

//     assert_eq!(
//         neighbours,
//         vec![UVec2::new(8191, 8191 - 1), UVec2::new(8191 - 1, 8191)]
//     );
//     assert_eq!(
//         neighbours_diag,
//         vec![
//             UVec2::new(8191 - 1, 8191 - 1),
//             UVec2::new(8191, 8191 - 1),
//             UVec2::new(8191 - 1, 8191)
//         ]
//     );
// }

// #[test]
// fn perf_testing_flat() {
//     let mut matrix = FlatMatrix::splat(UVec2::splat(8192), 0_u32);

//     {
//         let _timer = ScopedTimer::new("flat iter");

//         for (v, pos) in matrix.iter_with_pos_mut() {
//             if pos.x % 2 == 0 {
//                 *v += 1;
//             }
//         }
//     }

//     {
//         let _timer = ScopedTimer::new("flat set");

//         for pos in UVec2::splat(128).positions() {
//             matrix.set(pos, 69);
//         }
//     }

//     {
//         let _timer = ScopedTimer::new("flat neighbours");

//         for pos in UVec2::splat(16).positions() {
//             for n_pos in matrix.neighbours_with_diag(pos * 16) {
//                 matrix.set(n_pos, 69);
//             }
//         }
//     }
// }

// #[test]
// fn perf_testing_2d() {
//     let mut matrix = Matrix2D::splat(UVec2::splat(8192), 0_u8);

//     {
//         let _timer = ScopedTimer::new("2d iter");

//         for (v, pos) in matrix.iter_all_with_pos_mut() {
//             if pos.x % 2 == 0 {
//                 *v += 1;
//             }
//         }
//     }

//     {
//         let _timer = ScopedTimer::new("2d set");

//         for pos in UVec2::splat(128).positions() {
//             matrix.set(pos, 69);
//         }
//     }

//     {
//         let _timer = ScopedTimer::new("2d neighbours");

//         for pos in UVec2::splat(16).positions() {
//             for n_pos in matrix.neighbours(pos * 16, true) {
//                 matrix.set(n_pos, 69);
//             }
//         }
//     }
// }
