//! Precomputed constants for Falcon
//! 
//! This module contains precomputed mathematical constants:
//! - FFT roots and related constants
//! - NTT roots and modular arithmetic constants

pub mod fft_constants;
pub mod ntt_constants;

// Re-export specific items to avoid conflicts
pub use fft_constants::get_roots_dict as get_fft_roots_dict;
pub use ntt_constants::get_roots_dict as get_ntt_roots_dict;
pub use ntt_constants::get_inv_mod_q; 