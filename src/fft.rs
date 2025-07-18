// Falcon için FFT işlemleri (fft.py'den çevrildi)
use num_complex::Complex64;
use std::ops::Neg;
use crate::fft_constants::roots_dict;

pub fn split_fft(f_fft: &[Complex64]) -> (Vec<Complex64>, Vec<Complex64>) {
    let n = f_fft.len();
    let dict = roots_dict();
    let w = dict.get(&(n as u32)).expect("roots not found");
    let mut f0_fft = vec![Complex64::new(0.0, 0.0); n / 2];
    let mut f1_fft = vec![Complex64::new(0.0, 0.0); n / 2];
    for i in 0..n / 2 {
        f0_fft[i] = 0.5 * (f_fft[2 * i] + f_fft[2 * i + 1]);
        f1_fft[i] = 0.5 * (f_fft[2 * i] - f_fft[2 * i + 1]) * w[2 * i].1.neg();
    }
    (f0_fft, f1_fft)
}

pub fn merge_fft(f0_fft: &[Complex64], f1_fft: &[Complex64]) -> Vec<Complex64> {
    let n = 2 * f0_fft.len();
    let dict = roots_dict();
    let w = dict.get(&(n as u32)).expect("roots not found");
    let mut f_fft = vec![Complex64::new(0.0, 0.0); n];
    for i in 0..n / 2 {
        let w_c = Complex64::new(w[2 * i].0, w[2 * i].1);
        f_fft[2 * i] = f0_fft[i] + w_c * f1_fft[i];
        f_fft[2 * i + 1] = f0_fft[i] - w_c * f1_fft[i];
    }
    f_fft
}
// Diğer fonksiyonlar (fft, ifft, add, sub, mul, div, adj, add_fft, sub_fft, mul_fft, div_fft, adj_fft) gerektiğinde eklenebilir. 