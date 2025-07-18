//! Cryptographic operations for Falcon
//! 
//! This module contains cryptographic operations including:
//! - Main Falcon signature scheme
//! - NTRU key generation
//! - Signature encoding/decoding

pub mod falcon;
pub mod ntrugen;
pub mod encoding;

// Re-export commonly used items
pub use falcon::*;
pub use ntrugen::*;
pub use encoding::*; 