// Falcon ana parametreleri ve anahtar yapıları (falcon.py'den çevrildi)

use std::collections::HashMap;

pub const HEAD_LEN: usize = 1;
pub const SALT_LEN: usize = 40;
pub const SEED_LEN: usize = 56;

pub fn logn(n: usize) -> Option<usize> {
    match n {
        2 => Some(1),
        4 => Some(2),
        8 => Some(3),
        16 => Some(4),
        32 => Some(5),
        64 => Some(6),
        128 => Some(7),
        256 => Some(8),
        512 => Some(9),
        1024 => Some(10),
        _ => None,
    }
}

#[derive(Clone, Debug)]
pub struct FalconParams {
    pub n: usize,
    pub sigma: f64,
    pub sigmin: f64,
    pub sig_bound: usize,
    pub sig_bytelen: usize,
}

pub fn get_params() -> HashMap<usize, FalconParams> {
    let mut m = HashMap::new();
    m.insert(2, FalconParams { n: 2, sigma: 144.81253976308423, sigmin: 1.1165085072329104, sig_bound: 101498, sig_bytelen: 44 });
    m.insert(4, FalconParams { n: 4, sigma: 146.83798833523608, sigmin: 1.1321247692325274, sig_bound: 208714, sig_bytelen: 47 });
    m.insert(8, FalconParams { n: 8, sigma: 148.83587593064718, sigmin: 1.147528535373367, sig_bound: 428865, sig_bytelen: 52 });
    m.insert(16, FalconParams { n: 16, sigma: 151.78340713845503, sigmin: 1.170254078853483, sig_bound: 892039, sig_bytelen: 63 });
    m.insert(32, FalconParams { n: 32, sigma: 154.6747794602761, sigmin: 1.1925466358390344, sig_bound: 1852696, sig_bytelen: 82 });
    m.insert(64, FalconParams { n: 64, sigma: 157.51308555044122, sigmin: 1.2144300507766141, sig_bound: 3842630, sig_bytelen: 122 });
    m.insert(128, FalconParams { n: 128, sigma: 160.30114421975344, sigmin: 1.235926056771981, sig_bound: 7959734, sig_bytelen: 200 });
    m.insert(256, FalconParams { n: 256, sigma: 163.04153322607107, sigmin: 1.2570545284063217, sig_bound: 16468416, sig_bytelen: 356 });
    m.insert(512, FalconParams { n: 512, sigma: 165.7366171829776, sigmin: 1.2778336969128337, sig_bound: 34034726, sig_bytelen: 666 });
    m.insert(1024, FalconParams { n: 1024, sigma: 168.38857144654395, sigmin: 1.298280334344292, sig_bound: 70265242, sig_bytelen: 1280 });
    m
}

#[derive(Clone, Debug)]
pub struct PublicKey {
    pub n: usize,
    // Diğer alanlar gerektiğinde eklenir
}

#[derive(Clone, Debug)]
pub struct SecretKey {
    pub n: usize,
    // Diğer alanlar gerektiğinde eklenir
} 