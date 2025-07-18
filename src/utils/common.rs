// Falcon için ortak sabitler ve fonksiyonlar (common.py'den çevrildi)

/// Falcon'da kullanılan modülüs sabiti (q = 12 * 1024 + 1)
pub const Q: i32 = 12 * 1024 + 1;

/// Bir polinomu ikiye böler (çift ve tek indisler)
pub fn split<T: Copy>(f: &[T]) -> (Vec<T>, Vec<T>) {
    let n = f.len();
    let mut f0 = Vec::with_capacity(n / 2);
    let mut f1 = Vec::with_capacity(n / 2);
    for i in 0..n / 2 {
        f0.push(f[2 * i]);
        f1.push(f[2 * i + 1]);
    }
    (f0, f1)
}

/// İki polinomu birleştirir (merge)
pub fn merge<T: Copy + Default>(f_list: (&[T], &[T])) -> Vec<T> {
    let (f0, f1) = f_list;
    let n = 2 * f0.len();
    let mut f = vec![T::default(); n];
    for i in 0..f0.len() {
        f[2 * i] = f0[i];
        f[2 * i + 1] = f1[i];
    }
    f
}

/// Bir vektörün kare öklid normunu hesaplar (sqnorm)
pub fn sqnorm<T: Into<i64> + Copy>(v: &[Vec<T>]) -> i64 {
    let mut res = 0i64;
    for elt in v {
        for &coef in elt {
            res += coef.into() * coef.into();
        }
    }
    res
} 