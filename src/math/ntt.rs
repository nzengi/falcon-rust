// Falcon için NTT işlemleri (ntt.py'den çevrildi)
use crate::utils::common::Q;
use crate::constants::ntt_constants::{get_roots_dict, get_inv_mod_q};

const I2: u32 = 6145;

pub fn split_ntt(f_ntt: &[u32]) -> (Vec<u32>, Vec<u32>) {
    let n = f_ntt.len();
    let dict = get_roots_dict();
    let w = dict.get(&n).expect("roots not found");
    let inv_mod_q = get_inv_mod_q();
    let mut f0_ntt = vec![0u32; n / 2];
    let mut f1_ntt = vec![0u32; n / 2];
    for i in 0..n / 2 {
        let sum = (f_ntt[2 * i] as u64 + f_ntt[2 * i + 1] as u64) % Q as u64;
        f0_ntt[i] = ((I2 as u64 * sum) % Q as u64) as u32;
        
        let diff = (f_ntt[2 * i] as u64 + Q as u64 - f_ntt[2 * i + 1] as u64) % Q as u64;
        let prod = (I2 as u64 * diff * inv_mod_q[w[2 * i] as usize] as u64) % Q as u64;
        f1_ntt[i] = prod as u32;
    }
    (f0_ntt, f1_ntt)
}

pub fn merge_ntt(f0_ntt: &[u32], f1_ntt: &[u32]) -> Vec<u32> {
    let n = 2 * f0_ntt.len();
    let dict = get_roots_dict();
    let w = dict.get(&n).expect("roots not found");
    let mut f_ntt = vec![0u32; n];
    for i in 0..n / 2 {
        let prod = (w[2 * i] as u64 * f1_ntt[i] as u64) % Q as u64;
        f_ntt[2 * i] = ((f0_ntt[i] as u64 + prod) % Q as u64) as u32;
        f_ntt[2 * i + 1] = ((f0_ntt[i] as u64 + Q as u64 - prod) % Q as u64) as u32;
    }
    f_ntt
}

// Gerçek NTT implementasyonu
pub fn ntt(f: &[u32]) -> Vec<u32> {
    let n = f.len();
    if n == 1 {
        return f.to_vec();
    }
    
    if n == 2 {
        return vec![
            ((f[0] as u64 + f[1] as u64) % Q as u64) as u32,
            ((f[0] as u64 + Q as u64 - f[1] as u64) % Q as u64) as u32,
        ];
    }
    
    // Recursive NTT
    let mut f0 = vec![0u32; n / 2];
    let mut f1 = vec![0u32; n / 2];
    
    for i in 0..n / 2 {
        f0[i] = f[2 * i];
        f1[i] = f[2 * i + 1];
    }
    
    let f0_ntt = ntt(&f0);
    let f1_ntt = ntt(&f1);
    
    merge_ntt(&f0_ntt, &f1_ntt)
}

pub fn intt(f_ntt: &[u32]) -> Vec<u32> {
    let n = f_ntt.len();
    if n == 1 {
        return f_ntt.to_vec();
    }
    
    if n == 2 {
        return vec![
            ((I2 as u64 * (f_ntt[0] as u64 + f_ntt[1] as u64)) % Q as u64) as u32,
            ((I2 as u64 * (f_ntt[0] as u64 + Q as u64 - f_ntt[1] as u64)) % Q as u64) as u32,
        ];
    }
    
    let (f0_ntt, f1_ntt) = split_ntt(f_ntt);
    let f0 = intt(&f0_ntt);
    let f1 = intt(&f1_ntt);
    
    let mut f = vec![0u32; n];
    for i in 0..n / 2 {
        f[2 * i] = f0[i];
        f[2 * i + 1] = f1[i];
    }
    f
}

pub fn add_zq(f: &[u32], g: &[u32]) -> Vec<u32> {
    assert_eq!(f.len(), g.len());
    f.iter().zip(g.iter()).map(|(&a, &b)| ((a as u64 + b as u64) % Q as u64) as u32).collect()
}

pub fn neg_zq(f: &[u32]) -> Vec<u32> {
    f.iter().map(|&a| ((Q as u64 - a as u64) % Q as u64) as u32).collect()
}

pub fn sub_zq(f: &[u32], g: &[u32]) -> Vec<u32> {
    assert_eq!(f.len(), g.len());
    f.iter().zip(g.iter()).map(|(&a, &b)| ((a as u64 + Q as u64 - b as u64) % Q as u64) as u32).collect()
}

pub fn mul_zq(f: &[u32], g: &[u32]) -> Vec<u32> {
    let n = f.len();
    assert_eq!(n, g.len());
    
    let f_ntt = ntt(f);
    let g_ntt = ntt(g);
    let mut h_ntt = vec![0u32; n];
    
    for i in 0..n {
        h_ntt[i] = ((f_ntt[i] as u64 * g_ntt[i] as u64) % Q as u64) as u32;
    }
    
    intt(&h_ntt)
}

pub fn div_zq(f: &[u32], g: &[u32]) -> Result<Vec<u32>, &'static str> {
    let n = f.len();
    assert_eq!(n, g.len());
    
    let f_ntt = ntt(f);
    let g_ntt = ntt(g);
    let inv_mod_q = get_inv_mod_q();
    let mut h_ntt = vec![0u32; n];
    
    for i in 0..n {
        if g_ntt[i] == 0 {
            return Err("ZeroDivisionError");
        }
        h_ntt[i] = ((f_ntt[i] as u64 * inv_mod_q[g_ntt[i] as usize] as u64) % Q as u64) as u32;
    }
    
    Ok(intt(&h_ntt))
}

// NTT domain operations
pub fn add_ntt(f_ntt: &[u32], g_ntt: &[u32]) -> Vec<u32> {
    assert_eq!(f_ntt.len(), g_ntt.len());
    f_ntt.iter().zip(g_ntt.iter()).map(|(&a, &b)| (a + b) % Q as u32).collect()
}

pub fn sub_ntt(f_ntt: &[u32], g_ntt: &[u32]) -> Vec<u32> {
    assert_eq!(f_ntt.len(), g_ntt.len());
    f_ntt.iter().zip(g_ntt.iter()).map(|(&a, &b)| (a + Q as u32 - b) % Q as u32).collect()
}

pub fn mul_ntt(f_ntt: &[u32], g_ntt: &[u32]) -> Vec<u32> {
    assert_eq!(f_ntt.len(), g_ntt.len());
    f_ntt.iter().zip(g_ntt.iter()).map(|(&a, &b)| (a * b) % Q as u32).collect()
}

pub fn div_ntt(f_ntt: &[u32], g_ntt: &[u32]) -> Vec<u32> {
    assert_eq!(f_ntt.len(), g_ntt.len());
    let inv_mod_q = get_inv_mod_q();
    f_ntt.iter().zip(g_ntt.iter()).map(|(&a, &b)| {
        if b == 0 { panic!("ZeroDivisionError"); }
        (a * inv_mod_q[b as usize]) % Q as u32
    }).collect()
} 