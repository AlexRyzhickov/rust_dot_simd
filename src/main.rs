use rust_dot_simd::{dot, dot2, impl_score_dot_avx, read_bench_data};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    // let rows = read_bench_data("./file.txt").unwrap();
    // let vec: Vec<_> = rows.iter().rev().take(128).cloned().collect::<Vec<_>>().iter().rev().cloned().collect();
    // println!("{:?}", vec);
    // let mut dest: Vec<f32> = Vec::with_capacity(1024);
    // dest.resize(1024, 0.0);
    // dot(&rows, &vec);
    // dot2(&mut dest, &rows, &vec);
    // println!("Result: {:?}", dest);

    let query: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let vector: Vec<u8> = vec![8, 7, 6, 5, 4, 3, 2, 1];
    let dim = query.len() as u32;
    unsafe {
        let result = impl_score_dot_avx(query.as_ptr(), vector.as_ptr(), dim);
        println!("Result: {}", result);
        // Result: 120
    }
}
