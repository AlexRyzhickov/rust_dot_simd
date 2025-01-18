#[cfg(target_arch = "x86_64")]
extern "C" {
    fn impl_score_dot_avx(query_ptr: *const u8, vector_ptr: *const u8, dim: u32) -> f32;
}

fn main() {
    let query: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Пример данных
    let vector: Vec<u8> = vec![8, 7, 6, 5, 4, 3, 2, 1];
    let dim = query.len() as u32;
    unsafe {
        let result = impl_score_dot_avx(query.as_ptr(), vector.as_ptr(), dim);
        println!("Result: {}", result);
        // Result: 120
    }
}
