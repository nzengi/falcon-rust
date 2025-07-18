// Falcon için FFT işlemleri (fft.py'den çevrildi)
use num_complex::Complex64;
use std::ops::Neg;
use crate::fft_constants::get_roots_dict;

pub fn split_fft(f_fft: &[Complex64]) -> (Vec<Complex64>, Vec<Complex64>) {
    let n = f_fft.len();
    let dict = get_roots_dict();
    let w = dict.get(&n).expect("roots not found");
    let mut f0_fft = vec![Complex64::new(0.0, 0.0); n / 2];
    let mut f1_fft = vec![Complex64::new(0.0, 0.0); n / 2];
    for i in 0..n / 2 {
        f0_fft[i] = 0.5 * (f_fft[2 * i] + f_fft[2 * i + 1]);
        f1_fft[i] = 0.5 * (f_fft[2 * i] - f_fft[2 * i + 1]) * w[2 * i].conj();
    }
    (f0_fft, f1_fft)
}

pub fn merge_fft(f0_fft: &[Complex64], f1_fft: &[Complex64]) -> Vec<Complex64> {
    let n = 2 * f0_fft.len();
    let dict = get_roots_dict();
    let w = dict.get(&n).expect("roots not found");
    let mut f_fft = vec![Complex64::new(0.0, 0.0); n];
    for i in 0..n / 2 {
        f_fft[2 * i] = f0_fft[i] + w[2 * i] * f1_fft[i];
        f_fft[2 * i + 1] = f0_fft[i] - w[2 * i] * f1_fft[i];
    }
    f_fft
}

// Gerçek FFT implementasyonu
pub fn fft(f: &[f64]) -> Vec<Complex64> {
    let n = f.len();
    if n == 1 {
        return vec![Complex64::new(f[0], 0.0)];
    }
    
    if n == 2 {
        let f_fft = vec![
            Complex64::new(f[0] + f[1], 0.0),
            Complex64::new(f[0] - f[1], 0.0),
        ];
        return f_fft;
    }
    
    // Recursive FFT
    let mut f0 = vec![0.0; n / 2];
    let mut f1 = vec![0.0; n / 2];
    
    for i in 0..n / 2 {
        f0[i] = f[2 * i];
        f1[i] = f[2 * i + 1];
    }
    
    let f0_fft = fft(&f0);
    let f1_fft = fft(&f1);
    
    merge_fft(&f0_fft, &f1_fft)
}

pub fn ifft(f_fft: &[Complex64]) -> Vec<f64> {
    let n = f_fft.len();
    if n == 1 {
        return vec![f_fft[0].re];
    }
    
    if n == 2 {
        return vec![
            0.5 * (f_fft[0].re + f_fft[1].re),
            0.5 * (f_fft[0].re - f_fft[1].re),
        ];
    }
    
    let (f0_fft, f1_fft) = split_fft(f_fft);
    let f0 = ifft(&f0_fft);
    let f1 = ifft(&f1_fft);
    
    let mut f = vec![0.0; n];
    for i in 0..n / 2 {
        f[2 * i] = f0[i];
        f[2 * i + 1] = f1[i];
    }
    f
}

pub fn add(f: &[f64], g: &[f64]) -> Vec<f64> {
    assert_eq!(f.len(), g.len());
    f.iter().zip(g.iter()).map(|(a, b)| a + b).collect()
}

pub fn sub(f: &[f64], g: &[f64]) -> Vec<f64> {
    assert_eq!(f.len(), g.len());
    f.iter().zip(g.iter()).map(|(a, b)| a - b).collect()
}

pub fn neg(f: &[f64]) -> Vec<f64> {
    f.iter().map(|&x| -x).collect()
}

pub fn mul(f: &[f64], g: &[f64]) -> Vec<f64> {
    let n = f.len();
    assert_eq!(n, g.len());
    
    let f_fft = fft(f);
    let g_fft = fft(g);
    let mut h_fft = vec![Complex64::new(0.0, 0.0); n];
    
    for i in 0..n {
        h_fft[i] = f_fft[i] * g_fft[i];
    }
    
    ifft(&h_fft)
}

pub fn div(f: &[f64], g: &[f64]) -> Vec<f64> {
    let n = f.len();
    assert_eq!(n, g.len());
    
    let f_fft = fft(f);
    let g_fft = fft(g);
    let mut h_fft = vec![Complex64::new(0.0, 0.0); n];
    
    for i in 0..n {
        if g_fft[i].norm() > 1e-10 {
            h_fft[i] = f_fft[i] / g_fft[i];
        } else {
            h_fft[i] = Complex64::new(0.0, 0.0);
        }
    }
    
    ifft(&h_fft)
}

// FFT domain operations
pub fn add_fft(f_fft: &[Complex64], g_fft: &[Complex64]) -> Vec<Complex64> {
    assert_eq!(f_fft.len(), g_fft.len());
    f_fft.iter().zip(g_fft.iter()).map(|(a, b)| a + b).collect()
}

pub fn sub_fft(f_fft: &[Complex64], g_fft: &[Complex64]) -> Vec<Complex64> {
    assert_eq!(f_fft.len(), g_fft.len());
    f_fft.iter().zip(g_fft.iter()).map(|(a, b)| a - b).collect()
}

pub fn mul_fft(f_fft: &[Complex64], g_fft: &[Complex64]) -> Vec<Complex64> {
    assert_eq!(f_fft.len(), g_fft.len());
    f_fft.iter().zip(g_fft.iter()).map(|(a, b)| a * b).collect()
}

pub fn div_fft(f_fft: &[Complex64], g_fft: &[Complex64]) -> Vec<Complex64> {
    assert_eq!(f_fft.len(), g_fft.len());
    f_fft.iter().zip(g_fft.iter()).map(|(a, b)| {
        if b.norm() > 1e-10 {
            a / b
        } else {
            Complex64::new(0.0, 0.0)
        }
    }).collect()
}

pub fn adj_fft(f_fft: &[Complex64]) -> Vec<Complex64> {
    let n = f_fft.len();
    let mut result = vec![Complex64::new(0.0, 0.0); n];
    result[0] = f_fft[0].conj();
    for i in 1..n {
        result[i] = f_fft[n - i].conj();
    }
    result
}

pub fn adj(f: &[f64]) -> Vec<f64> {
    let n = f.len();
    let mut result = vec![0.0; n];
    result[0] = f[0];
    for i in 1..n {
        result[i] = f[n - i];
    }
    result
} 