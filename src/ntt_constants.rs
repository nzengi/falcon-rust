// Falcon için NTT sabitleri (ntt_constants.py'den çevrildi)
use std::collections::HashMap;

pub static PHI4_ROOTS_ZQ: [u32; 2] = [1479, 10810];
pub static PHI8_ROOTS_ZQ: [u32; 4] = [4043, 8246, 5146, 7143];

// ... diğer kök dizileri gerektiğinde eklenebilir ...

pub fn roots_dict_zq() -> HashMap<u32, &'static [u32]> {
    let mut m = HashMap::new();
    m.insert(2, &PHI4_ROOTS_ZQ[..]);
    m.insert(4, &PHI8_ROOTS_ZQ[..]);
    // Diğer kök dizileri gerektiğinde eklenebilir
    m
}

pub static INV_MOD_Q: [u32; 16] = [0, 1, 6145, 8193, 9217, 2458, 10241, 8778, 10753, 2731, 1229, 5586, 11265, 2836, 4389, 9012];
// ... INV_MOD_Q dizisinin tamamı gerektiğinde eklenebilir ... 