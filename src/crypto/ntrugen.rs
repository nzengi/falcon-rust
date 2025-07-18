// Falcon için NTRU anahtar üretimi ve yardımcı fonksiyonlar (ntrugen.py'den çevrildi)

pub fn karatsuba(a: &[i64], b: &[i64], n: usize) -> Vec<i64> {
    if n == 1 {
        vec![a[0] * b[0], 0]
    } else {
        let n2 = n / 2;
        let a0 = &a[..n2];
        let a1 = &a[n2..];
        let b0 = &b[..n2];
        let b1 = &b[n2..];
        let ax: Vec<i64> = a0.iter().zip(a1.iter()).map(|(&x, &y)| x + y).collect();
        let bx: Vec<i64> = b0.iter().zip(b1.iter()).map(|(&x, &y)| x + y).collect();
        let mut a0b0 = karatsuba(a0, b0, n2);
        let mut a1b1 = karatsuba(a1, b1, n2);
        let mut axbx = karatsuba(&ax, &bx, n2);
        for i in 0..n {
            axbx[i] -= a0b0[i] + a1b1[i];
        }
        let mut ab = vec![0; 2 * n];
        for i in 0..n {
            ab[i] += a0b0[i];
            ab[i + n] += a1b1[i];
            ab[i + n2] += axbx[i];
        }
        ab
    }
}

pub fn karamul(a: &[i64], b: &[i64]) -> Vec<i64> {
    let n = a.len();
    let ab = karatsuba(a, b, n);
    (0..n).map(|i| ab[i] - ab[i + n]).collect()
}

pub fn galois_conjugate(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    (0..n).map(|i| if i % 2 == 0 { a[i] } else { -a[i] }).collect()
}

pub fn field_norm(a: &[i64]) -> Vec<i64> {
    let n2 = a.len() / 2;
    let ae: Vec<i64> = (0..n2).map(|i| a[2 * i]).collect();
    let ao: Vec<i64> = (0..n2).map(|i| a[2 * i + 1]).collect();
    let ae_squared = karamul(&ae, &ae);
    let ao_squared = karamul(&ao, &ao);
    let mut res = ae_squared.clone();
    for i in 0..(n2 - 1) {
        res[i + 1] -= ao_squared[i];
    }
    res[0] += ao_squared[n2 - 1];
    res
}

pub fn lift(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut res = vec![0; 2 * n];
    for i in 0..n {
        res[2 * i] = a[i];
    }
    res
}

pub fn bitsize(a: i64) -> u32 {
    let mut val = a.abs();
    let mut res = 0;
    while val != 0 {
        res += 8;
        val >>= 8;
    }
    res
}

pub fn xgcd(mut b: i64, mut n: i64) -> (i64, i64, i64) {
    let (mut x0, mut x1, mut y0, mut y1) = (1, 0, 0, 1);
    while n != 0 {
        let q = b / n;
        let t = n;
        n = b % n;
        b = t;
        let t = x1;
        x1 = x0 - q * x1;
        x0 = t;
        let t = y1;
        y1 = y0 - q * y1;
        y0 = t;
    }
    (b, x0, y0)
} 