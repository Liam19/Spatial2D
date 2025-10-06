//! Benchmarks for Matrix type(s)

#![allow(missing_docs, reason = "self-explanitory")]

use spatial2d::*;

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn create_large_matrix() -> Matrix<u32> {
    let size = UVec2::splat(2048);
    Matrix::splat(size, 42)
}

fn create_small_matrix() -> Matrix<u32> {
    let size = UVec2::new(64, 64);
    Matrix::splat(size, 42)
}

fn bench_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix Creation");

    group.bench_function("new 64x64", |b| {
        b.iter(|| Matrix::<u32>::new(black_box(UVec2::splat(64))))
    });

    group.bench_function("new 2048x2048", |b| {
        b.iter(|| Matrix::<u32>::new(black_box(UVec2::splat(2048))))
    });

    group.bench_function("splat 64x64", |b| {
        b.iter(|| Matrix::splat(black_box(UVec2::splat(64)), black_box(42)))
    });

    group.bench_function("splat 2048x2048", |b| {
        b.iter(|| Matrix::splat(black_box(UVec2::splat(2048)), black_box(42)))
    });

    group.finish();
}

fn bench_access(c: &mut Criterion) {
    let matrix = create_large_matrix();
    let pos = UVec2::new(50, 50);

    c.bench_function("get", |b| b.iter(|| black_box(matrix.get(black_box(pos)))));

    let mut matrix = create_large_matrix();
    c.bench_function("get_mut", |b| {
        b.iter(|| {
            let val = black_box(matrix.get_mut(black_box(pos)));
            *val = black_box(24);
        })
    });

    c.bench_function("set", |b| {
        b.iter(|| matrix.set(black_box(pos), black_box(24)))
    });
}

fn bench_position_conversion(c: &mut Criterion) {
    let matrix = create_large_matrix();
    let pos = UVec2::new(50, 50);
    let idx = matrix.pos_to_idx(pos);

    c.bench_function("pos_to_idx", |b| {
        b.iter(|| black_box(matrix.pos_to_idx(black_box(pos))))
    });

    c.bench_function("idx_to_pos", |b| {
        b.iter(|| black_box(matrix.idx_to_pos(black_box(idx))))
    });
}

fn bench_iteration(c: &mut Criterion) {
    let matrix = create_large_matrix();
    let mut matrix_mut = create_large_matrix();

    let mut group = c.benchmark_group("Iteration");

    group.bench_function("iter", |b| {
        b.iter(|| {
            for item in matrix.iter() {
                black_box(item);
            }
        })
    });

    group.bench_function("iter_mut", |b| {
        b.iter(|| {
            for item in matrix_mut.iter_mut() {
                *item = black_box(24);
            }
        })
    });

    group.bench_function("iter_with_pos", |b| {
        b.iter(|| {
            for (val, pos) in matrix.iter_with_pos() {
                black_box((val, pos));
            }
        })
    });

    group.finish();
}

fn bench_neighbours(c: &mut Criterion) {
    let matrix = create_large_matrix();
    let center_pos = UVec2::splat(1024);
    let edge_pos = UVec2::splat(2048 - 1);

    let mut group = c.benchmark_group("Neighbours");

    group.bench_function("neighbours center", |b| {
        b.iter(|| black_box(matrix.neighbours_no_diag(black_box(center_pos))))
    });

    group.bench_function("neighbours edge", |b| {
        b.iter(|| black_box(matrix.neighbours_no_diag(black_box(edge_pos))))
    });

    group.bench_function("neighbours_with_diag center", |b| {
        b.iter(|| black_box(matrix.neighbours(black_box(center_pos))))
    });

    group.bench_function("neighbours_with_diag edge", |b| {
        b.iter(|| black_box(matrix.neighbours(black_box(edge_pos))))
    });

    group.bench_function("neighbours_radius 1", |b| {
        b.iter(|| black_box(matrix.neighbours_radius(black_box(center_pos), black_box(1))))
    });

    group.bench_function("neighbours_radius 2", |b| {
        b.iter(|| black_box(matrix.neighbours_radius(black_box(center_pos), black_box(2))))
    });

    group.bench_function("neighbours_radius 3", |b| {
        b.iter(|| black_box(matrix.neighbours_radius(black_box(center_pos), black_box(2))))
    });

    group.finish();
}

fn bench_analysis(c: &mut Criterion) {
    let matrix = create_large_matrix();

    c.bench_function("max_value", |b| b.iter(|| black_box(matrix.max_value())));

    c.bench_function("min_value", |b| b.iter(|| black_box(matrix.min_value())));

    c.bench_function("mean_value", |b| b.iter(|| black_box(matrix.mean_value())));
}

fn bench_extraction(c: &mut Criterion) {
    let matrix = create_large_matrix();

    let mut group = c.benchmark_group("Extraction");

    // Simple filter that matches about half the elements
    group.bench_function("extract_values (50% match)", |b| {
        b.iter(|| {
            let filter_fn = |x: &u32| *x % 2 == 0;
            for item in matrix.extract_values(filter_fn) {
                black_box(item);
            }
        })
    });

    // Filter that matches few elements
    group.bench_function("extract_values (10% match)", |b| {
        b.iter(|| {
            let filter_fn = |x: &u32| *x % 10 == 0;
            for item in matrix.extract_values(filter_fn) {
                black_box(item);
            }
        })
    });

    // Filter that matches most elements
    group.bench_function("extract_values (90% match)", |b| {
        b.iter(|| {
            let filter_fn = |x: &u32| *x % 10 != 0;
            for item in matrix.extract_values(filter_fn) {
                black_box(item);
            }
        })
    });

    // Position-based filters
    group.bench_function("extract_values_and_positions (x coord)", |b| {
        b.iter(|| {
            let filter_fn = |v: &u32, pos: UVec2| pos.x % 2 == 0;
            for item in matrix.extract_values_and_positions(filter_fn) {
                black_box(item);
            }
        })
    });

    group.bench_function("extract_positions (x coord)", |b| {
        b.iter(|| {
            let filter_fn = |v: &u32, pos: UVec2| pos.x % 2 == 0;
            for item in matrix.extract_positions(filter_fn) {
                black_box(item);
            }
        })
    });

    group.finish();
}

// - - - - - - - - - - - - - - - -
// - - - - - - - - - - - - - - - -
// - - - - - - - - - - - - - - - -
// - - - # # # # - - # # # # - - -
// - - - # # # # - - # # # # - - -
// - - - # # # # - - # # # # - - -
// - - - # # # # - - # # # # - - -
// # # # # # # # - - # # # # - - -
// - - - # # # # - - # # # # - - -
// - - - # # # # - - # # # # - - -
// - - - # # # # - - # # # # - - -
// - - - # # # # - - # # # # - - -
// - - - # # # # - - # # # # - - -
// - - - - - - - - - - - - - - - -
// - - - - - - - - - - - - - - - -
// - - - - - - - - - - - - - - - -
fn create_test_matrix(size: u32) -> Matrix<f32> {
    let mut matrix = Matrix::splat(UVec2::splat(size), 1.0);
    let half_size = size / 2;
    let gap = size / 8;
    let x_range_1 = (gap + 1)..(half_size - (gap - 1));
    let x_range_2 = (half_size + (gap - 1))..(size - (gap + 1));
    let y_range = (gap + 1)..(size - (gap + 1));

    for (v, pos) in matrix.iter_with_pos_mut() {
        if (x_range_1.contains(&pos.x) || x_range_2.contains(&pos.x)) && y_range.contains(&pos.y) {
            *v = 0.0;
        }
    }

    // Block left column
    for x in 0..(gap + 1) {
        matrix.set(UVec2::new(x, half_size), 0.0);
    }

    matrix
}

fn bench_a_star(c: &mut Criterion) {
    // Small grid benchmark
    let small_matrix = create_test_matrix(16);
    let small_start = UVec2::new(2, 2);
    let small_target = UVec2::new(13, 14);

    // Medium grid benchmark
    let medium_matrix = create_test_matrix(100);
    let medium_start = UVec2::new(5, 5);
    let medium_target = UVec2::new(92, 95);

    // Large grid benchmark
    let large_matrix = create_test_matrix(1000);
    let large_start = UVec2::new(50, 50);
    let large_target = UVec2::new(920, 950);

    // Simple move speed function
    let move_speed_fn = |_current: UVec2, _neighbor: UVec2, value: &f32| *value;

    let mut group = c.benchmark_group("A* Pathfinding");

    // Benchmark small grid
    group.bench_function("small_grid_no_diagonals", |b| {
        b.iter(|| {
            black_box(small_matrix.a_star_search(
                small_start,
                small_target,
                move_speed_fn,
                0.1,
                None,
                None,
            ))
        })
    });

    group.bench_function("small_grid_with_diagonals", |b| {
        b.iter(|| {
            black_box(small_matrix.a_star_search(
                small_start,
                small_target,
                move_speed_fn,
                0.1,
                None,
                Some(1.0),
            ))
        })
    });

    // Benchmark medium grid
    group.bench_function("medium_grid_no_diagonals", |b| {
        b.iter(|| {
            black_box(medium_matrix.a_star_search(
                medium_start,
                medium_target,
                move_speed_fn,
                0.1,
                None,
                None,
            ))
        })
    });

    group.bench_function("medium_grid_with_diagonals", |b| {
        b.iter(|| {
            black_box(medium_matrix.a_star_search(
                medium_start,
                medium_target,
                move_speed_fn,
                0.1,
                None,
                Some(1.0),
            ))
        })
    });

    // Benchmark large grid with search depth limit
    group.bench_function("large_grid_no_diagonals", |b| {
        b.iter(|| {
            black_box(large_matrix.a_star_search(
                large_start,
                large_target,
                move_speed_fn,
                0.1,
                Some(10_000_000),
                None,
            ))
        })
    });

    group.bench_function("large_grid_with_diagonals", |b| {
        b.iter(|| {
            black_box(large_matrix.a_star_search(
                large_start,
                large_target,
                move_speed_fn,
                0.1,
                Some(10_000_000),
                Some(1.0),
            ))
        })
    });

    // Benchmark with different move costs
    let complex_move_speed_fn = |current: UVec2, neighbor: UVec2, value: &f32| {
        let dist = (current.x.abs_diff(neighbor.x) + (current.y.abs_diff(neighbor.y))) as f32;
        *value / dist
    };

    group.bench_function("medium_grid_complex_cost", |b| {
        b.iter(|| {
            black_box(medium_matrix.a_star_search(
                medium_start,
                medium_target,
                complex_move_speed_fn,
                0.5,
                None,
                Some(1.0),
            ))
        })
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(32);
    targets = bench_a_star
);
// criterion_group!(
//     name = benches;
//     config = Criterion::default().sample_size(32);
//     targets = bench_creation, bench_access, bench_position_conversion,
//               bench_iteration, bench_neighbours, bench_analysis,
//               bench_extraction, bench_a_star
// );
criterion_main!(benches);
