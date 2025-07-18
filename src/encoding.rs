// Falcon için imza sıkıştırma ve açma fonksiyonları (encoding.py'den çevrildi)

pub fn compress(v: &[i32], slen: usize) -> Option<Vec<u8>> {
    let mut u = String::new();
    for &coef in v {
        // Encode the sign
        let s = if coef < 0 { "1" } else { "0" };
        // Encode the low bits
        let low = format!("{:07b}", (coef.abs() % (1 << 7)));
        // Encode the high bits
        let high = "0".repeat((coef.abs() >> 7) as usize) + "1";
        u += s;
        u += &low;
        u += &high;
    }
    if u.len() > 8 * slen {
        return None;
    }
    u += &"0".repeat(8 * slen - u.len());
    let mut w = Vec::with_capacity(slen);
    for i in 0..(u.len() / 8) {
        let byte = u[8 * i..8 * i + 8].chars().collect::<String>();
        w.push(u8::from_str_radix(&byte, 2).unwrap());
    }
    Some(w)
}

pub fn decompress(x: &[u8], slen: usize, n: usize) -> Option<Vec<i32>> {
    if x.len() > slen {
        return None;
    }
    let mut u = String::new();
    for &elt in x {
        u += &format!("{:08b}", elt);
    }
    let mut u = u;
    // Remove the last bits
    while u.ends_with('0') && !u.is_empty() {
        u.pop();
    }
    let mut v = Vec::with_capacity(n);
    let mut idx = 0;
    while idx < u.len() && v.len() < n {
        // Recover the sign
        let sign = if &u[idx..idx + 1] == "1" { -1 } else { 1 };
        // Recover the 7 low bits
        let low = i32::from_str_radix(&u[idx + 1..idx + 8], 2).ok()?;
        let mut i = idx + 8;
        let mut high = 0;
        while i < u.len() && &u[i..i + 1] == "0" {
            i += 1;
            high += 1;
        }
        if i >= u.len() {
            return None;
        }
        // Compute coef
        let coef = sign * (low + (high << 7));
        // Enforce a unique encoding for coef = 0
        if coef == 0 && sign == -1 {
            return None;
        }
        v.push(coef);
        idx = i + 1;
    }
    if v.len() != n {
        return None;
    }
    Some(v)
} 