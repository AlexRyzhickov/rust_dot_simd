use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(target_arch = "x86_64")]
extern "C" {
    pub fn impl_score_dot_avx(query_ptr: *const u8, vector_ptr: *const u8, dim: u32) -> f32;
}

pub fn read_bench_data(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut vec: Vec<u8> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let numbers: Result<Vec<u8>, _> = line.split(',').map(|s| s.trim().parse::<u8>()).collect();
        match numbers {
            Ok(mut nums) => vec.append(&mut nums),
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(vec)
}

pub fn dot(data: &Vec<u8>, vec: &Vec<u8>) {
    let step = 128;
    let dim = 128;
    for chunk_start in (0..data.len()).step_by(step) {
        let slice = &data[chunk_start..chunk_start + step];
        unsafe {
            let result = impl_score_dot_avx(slice.as_ptr(), vec.as_ptr(), dim);
            // println!("Result: {}", result);
            // Result: 120
        }
    }
}

pub fn dot2(dest: &mut Vec<f32>, data: &Vec<u8>, vec: &Vec<u8>) {
    let step = 128;
    let dim = 128;
    let mut count: usize = 0;
    for chunk_start in (0..data.len()).step_by(step) {
        let slice = &data[chunk_start..chunk_start + step];
        unsafe {
            let result = impl_score_dot_avx(slice.as_ptr(), vec.as_ptr(), dim);
            dest[count] = result;
            count += 1;
        }
    }
}
