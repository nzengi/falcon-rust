// Falcon için Gaussian örnekleyici (samplerz.py'den çevrildi)

use std::f64;

pub const MAX_SIGMA: f64 = 1.8205;
pub const INV_2SIGMA2: f64 = 1.0 / (2.0 * (MAX_SIGMA * MAX_SIGMA));
pub const RCDT_PREC: usize = 72;
pub const LN2: f64 = 0.69314718056;
pub const ILN2: f64 = 1.44269504089;

pub const RCDT: [u128; 18] = [
    3024686241123004913666,
    1564742784480091954050,
    636254429462080897535,
    199560484645026482916,
    47667343854657281903,
    8595902006365044063,
    1163297957344668388,
    117656387352093658,
    8867391802663976,
    496969357462633,
    20680885154299,
    638331848991,
    14602316184,
    247426747,
    3104126,
    28824,
    198,
    1
];

pub const C: [u64; 13] = [
    0x00000004741183A3,
    0x00000036548CFC06,
    0x0000024FDCBF140A,
    0x0000171D939DE045,
    0x0000D00CF58F6F84,
    0x000680681CF796E3,
    0x002D82D8305B0FEA,
    0x011111110E066FD0,
    0x0555555555070F00,
    0x155555555581FF00,
    0x400000000002B400,
    0x7FFFFFFFFFFF4800,
    0x8000000000000000
];

use rand::RngCore;

pub fn basesampler<R: RngCore>(rng: &mut R) -> u32 {
    let mut buf = [0u8; RCDT_PREC / 8];
    rng.fill_bytes(&mut buf);
    let u = u128::from_le_bytes({
        let mut arr = [0u8; 16];
        arr[..buf.len()].copy_from_slice(&buf);
        arr
    });
    let mut z0 = 0;
    for &elt in RCDT.iter() {
        if u < elt {
            z0 += 1;
        }
    }
    z0
}

pub fn approxexp(x: f64, ccs: f64) -> u64 {
    let mut y = C[0] as i128;
    let z = (x * (1u64 << 63) as f64) as i128;
    for &elt in &C[1..] {
        y = elt as i128 - ((z * y) >> 63);
    }
    let z2 = ((ccs * (1u64 << 63) as f64) as i128) << 1;
    let y2 = (z2 * y) >> 63;
    y2 as u64
}

pub fn berexp<R: RngCore>(x: f64, ccs: f64, rng: &mut R) -> bool {
    let mut s = (x * ILN2) as i32;
    let r = x - (s as f64) * LN2;
    s = s.min(63);
    let z = ((approxexp(r, ccs) - 1) >> s) as u64;
    let mut w = 0i32;
    for i in (0..=56).rev().step_by(8) {
        let p = rng.next_u32() as u8;
        w = p as i32 - (((z >> i) & 0xFF) as i32);
        if w != 0 {
            break;
        }
    }
    w < 0
}

pub fn samplerz<R: RngCore>(mu: f64, sigma: f64, sigmin: f64, rng: &mut R) -> i32 {
    use std::cmp::Ordering;
    let s = mu.floor() as i32;
    let r = mu - s as f64;
    let dss = 1.0 / (2.0 * sigma * sigma);
    let ccs = sigmin / sigma;
    loop {
        let z0 = basesampler(rng) as i32;
        let mut buf = [0u8; 1];
        rng.fill_bytes(&mut buf);
        let mut b = buf[0] & 1;
        let z = b as i32 + (2 * b as i32 - 1) * z0;
        let mut x = ((z as f64 - r).powi(2)) * dss;
        x -= (z0 as f64).powi(2) * INV_2SIGMA2;
        if berexp(x, ccs, rng) {
            return z + s;
        }
    }
} 