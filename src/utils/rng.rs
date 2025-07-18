// Falcon için ChaCha20 tabanlı PRNG (rng.py'den çevrildi)

const CW: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

fn roll(x: u32, n: u32) -> u32 {
    ((x << n) & 0xffffffff) | (x >> (32 - n))
}

pub struct ChaCha20 {
    s: [u32; 14],
    ctr: u64,
    hexbytes: Vec<u8>,
    state: [u32; 16],
}

impl ChaCha20 {
    pub fn new(src: &[u8]) -> Self {
        assert!(src.len() >= 56);
        let mut s = [0u32; 14];
        for i in 0..14 {
            s[i] = u32::from_le_bytes([src[4 * i], src[4 * i + 1], src[4 * i + 2], src[4 * i + 3]]);
        }
        let ctr = s[12] as u64 + ((s[13] as u64) << 32);
        Self {
            s,
            ctr,
            hexbytes: Vec::new(),
            state: [0u32; 16],
        }
    }

    fn qround(&mut self, a: usize, b: usize, c: usize, d: usize) {
        let mut a_ = self.state[a];
        let mut b_ = self.state[b];
        let mut c_ = self.state[c];
        let mut d_ = self.state[d];
        a_ = a_.wrapping_add(b_);
        d_ = roll(d_ ^ a_, 16);
        c_ = c_.wrapping_add(d_);
        b_ = roll(b_ ^ c_, 12);
        a_ = a_.wrapping_add(b_);
        d_ = roll(d_ ^ a_, 8);
        c_ = c_.wrapping_add(d_);
        b_ = roll(b_ ^ c_, 7);
        self.state[a] = a_;
        self.state[b] = b_;
        self.state[c] = c_;
        self.state[d] = d_;
    }

    fn update(&mut self) -> [u32; 16] {
        self.state = [0u32; 16];
        self.state[0..4].copy_from_slice(&CW);
        for i in 0..10 {
            self.state[4 + i] = self.s[i];
        }
        self.state[14] = self.s[10] ^ (self.ctr as u32);
        self.state[15] = self.s[11] ^ ((self.ctr >> 32) as u32);
        let mut working_state = self.state;
        for _ in 0..10 {
            self.qround(0, 4, 8, 12);
            self.qround(1, 5, 9, 13);
            self.qround(2, 6, 10, 14);
            self.qround(3, 7, 11, 15);
            self.qround(0, 5, 10, 15);
            self.qround(1, 6, 11, 12);
            self.qround(2, 7, 8, 13);
            self.qround(3, 4, 9, 14);
        }
        for i in 0..16 {
            self.state[i] = self.state[i].wrapping_add(working_state[i]);
        }
        self.ctr += 1;
        self.state
    }

    fn block_update(&mut self) -> Vec<u8> {
        let mut block = vec![0u32; 16 * 8];
        for i in 0..8 {
            let update = self.update();
            for j in 0..16 {
                block[i + 8 * j] = update[j];
            }
        }
        let mut out = Vec::with_capacity(16 * 8 * 4);
        for word in block {
            out.extend_from_slice(&word.to_le_bytes());
        }
        out
    }

    pub fn randombytes(&mut self, k: usize) -> Vec<u8> {
        if k > self.hexbytes.len() {
            self.hexbytes = self.block_update();
        }
        let out = self.hexbytes[..k].to_vec();
        self.hexbytes = self.hexbytes[k..].to_vec();
        out
    }
} 