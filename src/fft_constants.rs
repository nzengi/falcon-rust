// Falcon için FFT sabitleri (fft_constants.py'den çevrildi)
use std::collections::HashMap;

pub static PHI4_ROOTS: [(f64, f64); 2] = [
    (0.0, 1.0),
    (0.0, -1.0),
];

pub static PHI8_ROOTS: [(f64, f64); 4] = [
    (0.707106781186548, 0.707106781186547),
    (-0.707106781186548, -0.707106781186547),
    (0.707106781186548, -0.707106781186547),
    (-0.707106781186548, 0.707106781186547),
];

pub fn roots_dict() -> HashMap<u32, &'static [(f64, f64)]> {
    let mut m = HashMap::new();
    m.insert(2, &PHI4_ROOTS[..]);
    m.insert(4, &PHI8_ROOTS[..]);
    // Diğer kök dizileri gerektiğinde eklenebilir
    m
} 