pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod common;
pub mod rng;
pub mod samplerz;
pub mod fft_constants;
pub mod ntt_constants;
pub mod ntt;
pub mod fft;
pub mod ntrugen;
pub mod ffsampling;