// //! Benchmarks for Direction type

// #![allow(missing_docs, reason = "self-explanitory")]

// use spatial2d::*;

// use criterion::{Criterion, criterion_group, criterion_main};
// use std::hint::black_box;

// fn bench_creation(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Matrix Creation");

//     group.bench_function("new 64x64", |b| {
//         b.iter(|| Matrix::<u32>::new(black_box(UVec2::splat(64))))
//     });

//     group.bench_function("new 2048x2048", |b| {
//         b.iter(|| Matrix::<u32>::new(black_box(UVec2::splat(2048))))
//     });

//     group.bench_function("splat 64x64", |b| {
//         b.iter(|| Matrix::splat(black_box(UVec2::splat(64)), black_box(42)))
//     });

//     group.bench_function("splat 2048x2048", |b| {
//         b.iter(|| Matrix::splat(black_box(UVec2::splat(2048)), black_box(42)))
//     });

//     group.finish();
// }

// fn bench_iteration(c: &mut Criterion) {
//     let matrix = create_large_matrix();
//     let mut matrix_mut = create_large_matrix();

//     let mut group = c.benchmark_group("Iteration");

//     group.bench_function("iter", |b| {
//         b.iter(|| {
//             for item in matrix.iter() {
//                 black_box(item);
//             }
//         })
//     });

//     group.bench_function("iter_mut", |b| {
//         b.iter(|| {
//             for item in matrix_mut.iter_mut() {
//                 *item = black_box(24);
//             }
//         })
//     });

//     group.bench_function("iter_with_pos", |b| {
//         b.iter(|| {
//             for (val, pos) in matrix.iter_with_pos() {
//                 black_box((val, pos));
//             }
//         })
//     });

//     group.finish();
// }

// criterion_group!(
//     name = benches;
//     config = Criterion::default().sample_size(32);
//     targets = bench_a_star
// );
// // criterion_group!(
// //     name = benches;
// //     config = Criterion::default().sample_size(32);
// //     targets = bench_creation, bench_access, bench_position_conversion,
// //               bench_iteration, bench_neighbours, bench_analysis,
// //               bench_extraction, bench_a_star
// // );
// criterion_main!(benches);
