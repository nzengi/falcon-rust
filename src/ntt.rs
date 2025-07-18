// Falcon için NTT işlemleri (ntt.py'den çevrildi)
use crate::common::Q;
use crate::ntt_constants::{roots_dict_zq, INV_MOD_Q};

const I2: u32 = 6145;

pub fn split_ntt(f_ntt: &[u32]) -> (Vec<u32>, Vec<u32>) {
    let n = f_ntt.len();
    let dict = roots_dict_zq();
    let w = dict.get(&(n as u32)).expect("roots not found");
    let mut f0_ntt = vec![0u32; n / 2];
    let mut f1_ntt = vec![0u32; n / 2];
    for i in 0..n / 2 {
        f0_ntt[i] = (I2 * (f_ntt[2 * i] + f_ntt[2 * i + 1])) % Q as u32;
        f1_ntt[i] = (I2 * (f_ntt[2 * i] - f_ntt[2 * i + 1]) * INV_MOD_Q[w[2 * i] as usize]) % Q as u32;
    }
    (f0_ntt, f1_ntt)
}

pub fn merge_ntt(f0_ntt: &[u32], f1_ntt: &[u32]) -> Vec<u32> {
    let n = 2 * f0_ntt.len();
    let dict = roots_dict_zq();
    let w = dict.get(&(n as u32)).expect("roots not found");
    let mut f_ntt = vec![0u32; n];
    for i in 0..n / 2 {
        f_ntt[2 * i] = (f0_ntt[i] + w[2 * i] * f1_ntt[i]) % Q as u32;
        f_ntt[2 * i + 1] = (f0_ntt[i] - w[2 * i] * f1_ntt[i]) % Q as u32;
    }
    f_ntt
}

pub fn add_zq(f: &[u32], g: &[u32]) -> Vec<u32> {
    assert_eq!(f.len(), g.len());
    f.iter().zip(g.iter()).map(|(&a, &b)| (a + b) % Q as u32).collect()
}

pub fn neg_zq(f: &[u32]) -> Vec<u32> {
    f.iter().map(|&a| (-((a as i32)) as u32) % Q as u32).collect()
}

pub fn sub_zq(f: &[u32], g: &[u32]) -> Vec<u32> {
    add_zq(f, &neg_zq(g))
}

pub fn mul_ntt(f_ntt: &[u32], g_ntt: &[u32]) -> Vec<u32> {
    assert_eq!(f_ntt.len(), g_ntt.len());
    f_ntt.iter().zip(g_ntt.iter()).map(|(&a, &b)| (a * b) % Q as u32).collect()
}

pub fn div_ntt(f_ntt: &[u32], g_ntt: &[u32]) -> Vec<u32> {
    assert_eq!(f_ntt.len(), g_ntt.len());
    f_ntt.iter().zip(g_ntt.iter()).map(|(&a, &b)| {
        if b == 0 { panic!("ZeroDivisionError"); }
        (a * INV_MOD_Q[b as usize]) % Q as u32
    }).collect()
}
// Diğer fonksiyonlar (ntt, intt, add_ntt, sub_ntt, mul_zq, div_zq) gerektiğinde eklenebilir. 