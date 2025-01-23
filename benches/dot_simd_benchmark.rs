use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_dot_simd::{dot, dot2, read_bench_data};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot_simd_group");
    let rows = read_bench_data("./file.txt").unwrap();
    let vec: Vec<_> = rows
        .iter()
        .rev()
        .take(128)
        .cloned()
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .cloned()
        .collect();

    group.bench_function("dot_simd", |b| {
        b.iter(|| dot(black_box(&rows), black_box(&vec)))
    });
    group.bench_function("dot2_simd", |b| {
        b.iter(|| {
            let mut dest: Vec<f32> = Vec::with_capacity(1024);
            dest.resize(1024, 0.0);
            black_box(&mut dest);
            dot2(&mut dest, black_box(&rows), black_box(&vec))
        });
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// Bench results
// fn quick_select for structures without generics is faster than fn quick_select_no_generic with the generic: 13.339 ms vs 23.555 ms
// fn quick_select for primitive types without generics has the same performance as fn quick_select_no_generic with the generic

// Running benches/dot_simd_benchmark.rs (target/release/deps/dot_simd_benchmark-7271deb2ff1823de)
// Gnuplot not found, using plotters backend
// dot_simd_group/dot_simd time:   [3.1183 µs 3.1197 µs 3.1210 µs]
// change: [-0.8804% -0.7046% -0.5122%] (p = 0.00 < 0.05)
// Change within noise threshold.
// Found 9 outliers among 100 measurements (9.00%)
// 3 (3.00%) low mild
// 3 (3.00%) high mild
// 3 (3.00%) high severe
// dot_simd_group/dot2_simd
// time:   [3.2819 µs 3.2839 µs 3.2858 µs]
// change: [-2.7416% -2.2304% -1.7496%] (p = 0.00 < 0.05)
// Performance has improved.
// Found 5 outliers among 100 measurements (5.00%)
// 1 (1.00%) low mild
// 1 (1.00%) high mild
// 3 (3.00%) high severe
