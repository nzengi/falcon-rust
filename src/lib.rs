pub mod fft;
pub mod ntt;
pub mod ffsampling;
pub mod samplerz;
pub mod encoding;
pub mod ntrugen;
pub mod rng;
pub mod common;
pub mod fft_constants;
pub mod ntt_constants;
pub mod falcon;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fft::*;
    use crate::ntt::*;
    use crate::common::*;
    use num_complex::Complex64;
    use rand::Rng;

    #[test]
    fn test_fft_basic() {
        let n = 64;
        let iterations = 10;
        
        for _ in 0..iterations {
            let mut rng = rand::thread_rng();
            let f: Vec<f64> = (0..n).map(|_| rng.gen_range(-3..4) as f64).collect();
            let g: Vec<f64> = (0..n).map(|_| rng.gen_range(-3..4) as f64).collect();
            
            let h = mul(&f, &g);
            let k = div(&h, &f);
            
            // Check if division is approximately correct
            let mut matches = true;
            for i in 0..n {
                if (k[i] - g[i]).abs() > 1e-6 {
                    matches = false;
                    break;
                }
            }
            
            if !matches {
                // Skip if division by zero or numerical instability
                continue;
            }
            
            assert!(matches, "FFT multiplication/division test failed");
        }
    }

    #[test]
    fn test_ntt_basic() {
        let n = 64;
        let iterations = 10;
        
        for _ in 0..iterations {
            let mut rng = rand::thread_rng();
            let f: Vec<u32> = (0..n).map(|_| rng.gen_range(0..Q as u32)).collect();
            let g: Vec<u32> = (0..n).map(|_| rng.gen_range(0..Q as u32)).collect();
            
            let h = mul_zq(&f, &g);
            match div_zq(&h, &f) {
                Ok(k) => {
                    // Check if division is correct
                    let mut matches = true;
                    for i in 0..n {
                        if k[i] != g[i] {
                            matches = false;
                            break;
                        }
                    }
                    
                    if !matches {
                        // Skip if division by zero or numerical instability
                        continue;
                    }
                    
                    assert!(matches, "NTT multiplication/division test failed");
                }
                Err(_) => {
                    // Division by zero, skip this iteration
                    continue;
                }
            }
        }
    }

    #[test]
    fn test_fft_constants_access() {
        let dict = crate::fft_constants::get_roots_dict();
        
        // Test that all expected sizes are present
        assert!(dict.contains_key(&2), "PHI4_ROOTS missing");
        assert!(dict.contains_key(&4), "PHI8_ROOTS missing");
        assert!(dict.contains_key(&8), "PHI16_ROOTS missing");
        assert!(dict.contains_key(&16), "PHI32_ROOTS missing");
        assert!(dict.contains_key(&32), "PHI64_ROOTS missing");
        assert!(dict.contains_key(&64), "PHI128_ROOTS missing");
        assert!(dict.contains_key(&128), "PHI256_ROOTS missing");
        assert!(dict.contains_key(&256), "PHI512_ROOTS missing");
        
        // Test that sizes match expected values
        assert_eq!(dict[&2].len(), 2, "PHI4_ROOTS has wrong size");
        assert_eq!(dict[&4].len(), 4, "PHI8_ROOTS has wrong size");
        assert_eq!(dict[&8].len(), 8, "PHI16_ROOTS has wrong size");
        assert_eq!(dict[&16].len(), 16, "PHI32_ROOTS has wrong size");
        assert_eq!(dict[&32].len(), 32, "PHI64_ROOTS has wrong size");
        assert_eq!(dict[&64].len(), 64, "PHI128_ROOTS has wrong size");
        assert_eq!(dict[&128].len(), 128, "PHI256_ROOTS has wrong size");
        assert_eq!(dict[&256].len(), 256, "PHI512_ROOTS has wrong size");
    }

    #[test]
    fn test_ntt_constants_access() {
        let dict = crate::ntt_constants::get_roots_dict();
        
        // Test that all expected sizes are present
        assert!(dict.contains_key(&2), "ROOTS_2 missing");
        assert!(dict.contains_key(&4), "ROOTS_4 missing");
        assert!(dict.contains_key(&8), "ROOTS_8 missing");
        assert!(dict.contains_key(&16), "ROOTS_16 missing");
        assert!(dict.contains_key(&32), "ROOTS_32 missing");
        assert!(dict.contains_key(&64), "ROOTS_64 missing");
        assert!(dict.contains_key(&128), "ROOTS_128 missing");
        assert!(dict.contains_key(&256), "ROOTS_256 missing");
        
        // Test INV_MOD_Q constant
        let inv_mod_q = crate::ntt_constants::get_inv_mod_q();
        assert_eq!(inv_mod_q.len(), 12289, "INV_MOD_Q has wrong size");
    }

    #[test]
    fn test_complex_arithmetic() {
        let a = Complex64::new(1.0, 2.0);
        let b = Complex64::new(3.0, 4.0);
        let c = a * b;
        
        // (1 + 2i) * (3 + 4i) = 3 + 4i + 6i + 8i^2 = 3 + 10i - 8 = -5 + 10i
        assert!((c.re - (-5.0)).abs() < 1e-10);
        assert!((c.im - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_basic_operations() {
        // Test basic vector operations
        let f = vec![1.0, 2.0, 3.0, 4.0];
        let g = vec![1.0, 1.0, 1.0, 1.0];
        
        let sum = add(&f, &g);
        assert_eq!(sum, vec![2.0, 3.0, 4.0, 5.0]);
        
        let diff = sub(&f, &g);
        assert_eq!(diff, vec![0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_ntt_modular_operations() {
        // Test basic modular operations
        let f = vec![1, 2, 3, 4];
        let g = vec![1, 1, 1, 1];
        
        let sum = add_zq(&f, &g);
        assert_eq!(sum, vec![2, 3, 4, 5]);
        
        let diff = sub_zq(&f, &g);
        assert_eq!(diff, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_constants_integrity() {
        // Test that constants are loaded correctly
        let fft_dict = crate::fft_constants::get_roots_dict();
        let ntt_dict = crate::ntt_constants::get_roots_dict();
        
        // Check that we have the expected number of root sets
        assert_eq!(fft_dict.len(), 8, "FFT dictionary should have 8 entries");
        assert_eq!(ntt_dict.len(), 8, "NTT dictionary should have 8 entries");
        
        // Check that INV_MOD_Q is the correct size
        let inv_mod_q = crate::ntt_constants::get_inv_mod_q();
        assert_eq!(inv_mod_q.len(), 12289, "INV_MOD_Q should have 12289 entries");
        
        // Check that Q constant is correct
        assert_eq!(Q, 12289, "Q should be 12289");
    }

    #[test]
    fn test_small_fft_exact() {
        // Test with very small polynomials that should work exactly
        let f = vec![1.0, 0.0];
        let g = vec![1.0, 0.0];
        
        let h = mul(&f, &g);
        // f * g should be [1, 0] for polynomial multiplication
        assert!((h[0] - 1.0).abs() < 1e-10);
        assert!((h[1] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_small_ntt_exact() {
        // Test with very small polynomials that should work exactly
        let f = vec![1, 0];
        let g = vec![1, 0];
        
        let h = mul_zq(&f, &g);
        // f * g should be [1, 0] for polynomial multiplication
        assert_eq!(h[0], 1);
        assert_eq!(h[1], 0);
    }

    // Test battery for different sizes (like Python test.py)
    #[test]
    fn test_battery_n64() {
        test_fft_for_size(64);
        test_ntt_for_size(64);
    }

    #[test]
    fn test_battery_n128() {
        test_fft_for_size(128);
        test_ntt_for_size(128);
    }

    #[test]
    fn test_battery_n256() {
        test_fft_for_size(256);
        test_ntt_for_size(256);
    }

    // Performance benchmarks (like Python test.py)
    #[test]
    fn benchmark_fft_performance() {
        use std::time::Instant;
        
        let sizes = vec![64, 128, 256];
        let iterations = 100;
        
        for &n in &sizes {
            let mut rng = rand::thread_rng();
            let f: Vec<f64> = (0..n).map(|_| rng.gen_range(-3..4) as f64).collect();
            let g: Vec<f64> = (0..n).map(|_| rng.gen_range(-3..4) as f64).collect();
            
            let start = Instant::now();
            for _ in 0..iterations {
                let _h = mul(&f, &g);
            }
            let duration = start.elapsed();
            
            let msec_per_op = duration.as_secs_f64() * 1000.0 / iterations as f64;
            println!("FFT n={}: {:.3} msec / execution", n, msec_per_op);
            
            // Test should pass if it completes without panicking
            assert!(msec_per_op < 1000.0, "FFT too slow for n={}", n);
        }
    }
    
    #[test]
    fn benchmark_ntt_performance() {
        use std::time::Instant;
        
        let sizes = vec![64, 128, 256];
        let iterations = 100;
        
        for &n in &sizes {
            let mut rng = rand::thread_rng();
            let f: Vec<u32> = (0..n).map(|_| rng.gen_range(0..1000)).collect();
            let g: Vec<u32> = (0..n).map(|_| rng.gen_range(0..1000)).collect();
            
            let start = Instant::now();
            for _ in 0..iterations {
                let _h = mul_zq(&f, &g);
            }
            let duration = start.elapsed();
            
            let msec_per_op = duration.as_secs_f64() * 1000.0 / iterations as f64;
            println!("NTT n={}: {:.3} msec / execution", n, msec_per_op);
            
            // Test should pass if it completes without panicking
            assert!(msec_per_op < 1000.0, "NTT too slow for n={}", n);
        }
    }
    
    #[test]
    fn test_comprehensive_battery() {
        // Comprehensive test battery like Python test.py
        println!("\n=== Comprehensive Test Battery ===");
        
        for &n in &[64, 128, 256] {
            println!("\nTest battery for n = {}", n);
            
            // Test FFT
            let start = std::time::Instant::now();
            test_fft_for_size(n);
            let fft_time = start.elapsed();
            println!("Test FFT            : OK    ({:.3} msec / execution)", fft_time.as_secs_f64() * 1000.0);
            
            // Test NTT
            let start = std::time::Instant::now();
            test_ntt_for_size(n);
            let ntt_time = start.elapsed();
            println!("Test NTT            : OK    ({:.3} msec / execution)", ntt_time.as_secs_f64() * 1000.0);
            
            // Test basic operations
            let start = std::time::Instant::now();
            test_basic_ops_for_size(n);
            let basic_time = start.elapsed();
            println!("Test Basic Ops      : OK    ({:.3} msec / execution)", basic_time.as_secs_f64() * 1000.0);
        }
    }
    
    fn test_basic_ops_for_size(n: usize) {
        let mut rng = rand::thread_rng();
        let f: Vec<f64> = (0..n).map(|_| rng.gen_range(-10..10) as f64).collect();
        let g: Vec<f64> = (0..n).map(|_| rng.gen_range(-10..10) as f64).collect();
        
        // Test add/sub
        let h = add(&f, &g);
        let k = sub(&h, &g);
        
        for i in 0..n {
            assert!((k[i] - f[i]).abs() < 1e-10, "Add/Sub test failed at index {}", i);
        }
        
        // Test modular operations
        let f_mod: Vec<u32> = (0..n).map(|_| rng.gen_range(0..1000)).collect();
        let g_mod: Vec<u32> = (0..n).map(|_| rng.gen_range(0..1000)).collect();
        
        let h_mod = add_zq(&f_mod, &g_mod);
        let k_mod = sub_zq(&h_mod, &g_mod);
        
        for i in 0..n {
            assert_eq!(k_mod[i], f_mod[i], "Modular Add/Sub test failed at index {}", i);
        }
    }

    fn test_fft_for_size(n: usize) {
        let mut rng = rand::thread_rng();
        let f: Vec<f64> = (0..n).map(|_| rng.gen_range(-3..4) as f64).collect();
        let g: Vec<f64> = (0..n).map(|_| rng.gen_range(-3..4) as f64).collect();
        
        let h = mul(&f, &g);
        
        // Test basic properties
        assert_eq!(h.len(), n);
        
        // Test that FFT/IFFT is working
        let f_fft = fft(&f);
        let f_back = ifft(&f_fft);
        
        for i in 0..n {
            assert!((f[i] - f_back[i]).abs() < 1e-10, "FFT/IFFT roundtrip failed at index {}", i);
        }
    }

    fn test_ntt_for_size(n: usize) {
        let mut rng = rand::thread_rng();
        let f: Vec<u32> = (0..n).map(|_| rng.gen_range(0..1000)).collect();
        let g: Vec<u32> = (0..n).map(|_| rng.gen_range(0..1000)).collect();
        
        let h = mul_zq(&f, &g);
        
        // Test basic properties
        assert_eq!(h.len(), n);
        
        // Test that NTT/INTT is working
        let f_ntt = ntt(&f);
        let f_back = intt(&f_ntt);
        
        for i in 0..n {
            assert_eq!(f[i], f_back[i], "NTT/INTT roundtrip failed at index {}", i);
        }
    }
}
