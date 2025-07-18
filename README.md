# Falcon-Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-red.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

A Rust implementation of the Falcon post-quantum cryptographic signature scheme.

Falcon stands for **FA**st Fourier **L**attice-based **CO**mpact signatures over **N**TRU. This repository implements the signature scheme as described in [https://falcon-sign.info/](https://falcon-sign.info/) using the Rust programming language.

## 🚀 Quick Start

```rust
use falcon_rust::*;

fn main() {
    // Run FFT and NTT tests
    println!("Starting Falcon-Rust test suite...");

    // Test basic operations
    let n = 64;
    println!("Running test battery for n={}", n);

    // Performance tests
    run_performance_tests();
}

fn run_performance_tests() {
    for &n in &[64, 128, 256] {
        println!("\nTest battery for n = {}", n);

        // FFT tests
        test_fft_performance(n);

        // NTT tests
        test_ntt_performance(n);

        // Basic operation tests
        test_basic_operations(n);
    }
}
```

## 📋 Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Performance](#performance)
- [Tests](#tests)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## 🛠️ Installation

### Rust Requirements

- Rust 1.70 or higher
- Cargo package manager

### Cloning and Building the Project

```bash
# Clone the repository
git clone https://github.com/your-username/falcon-rust.git
cd falcon-rust

# Build the project
cargo build --release

# Run tests
cargo test

# Run performance tests
cargo test --release test_battery -- --nocapture
```

## 🎯 Usage

### Basic Operations

Falcon-Rust includes the following main components:

1. **FFT (Fast Fourier Transform)**: Fast transform over `R[x] / (x^n + 1)`
2. **NTT (Number Theoretic Transform)**: Number theoretic transform over `Z_q[x] / (x^n + 1)`
3. **Gaussian Sampling**: Gaussian sampling for lattice-based cryptography
4. **NTRU Key Generation**: NTRU-based key pair generation
5. **Fast Fourier Sampling**: Fast Fourier sampling algorithm

### Running Tests

```bash
# Run all tests
cargo test

# Run only performance tests
cargo test test_battery

# Run with detailed output
cargo test -- --nocapture

# Run in release mode (optimized)
cargo test --release
```

Expected test output:

```
Test battery for n = 64
Test FFT            : OK    (0.xxx msec / execution)
Test NTT            : OK    (0.xxx msec / execution)
Test Basic Ops      : OK    (0.xxx msec / execution)

Test battery for n = 128
Test FFT            : OK    (0.xxx msec / execution)
Test NTT            : OK    (0.xxx msec / execution)
Test Basic Ops      : OK    (0.xxx msec / execution)

Test battery for n = 256
Test FFT            : OK    (0.xxx msec / execution)
Test NTT            : OK    (0.xxx msec / execution)
Test Basic Ops      : OK    (0.xxx msec / execution)
```

## 📁 Project Structure

```
falcon-rust/
├── src/
│   ├── lib.rs             # Main library entry point and tests
│   ├── constants/         # Precomputed mathematical constants
│   │   ├── mod.rs         # Constants module exports
│   │   ├── fft_constants.rs    # FFT roots and related constants
│   │   └── ntt_constants.rs    # NTT roots and modular constants
│   ├── crypto/            # Cryptographic operations
│   │   ├── mod.rs         # Crypto module exports
│   │   ├── falcon.rs      # Main Falcon signature scheme
│   │   ├── ntrugen.rs     # NTRU key generation
│   │   └── encoding.rs    # Signature encoding/decoding
│   ├── math/              # Mathematical operations
│   │   ├── mod.rs         # Math module exports
│   │   ├── fft.rs         # FFT over R[x] / (x^n + 1)
│   │   ├── ntt.rs         # NTT over Z_q[x] / (x^n + 1)
│   │   ├── ffsampling.rs  # Fast Fourier sampling
│   │   └── samplerz.rs    # Gaussian sampling over integers
│   ├── utils/             # Utility functions
│   │   ├── mod.rs         # Utils module exports
│   │   ├── common.rs      # Common functions and constants
│   │   └── rng.rs         # Random number generation (ChaCha20)
│   └── tests/             # Additional test files (future)
├── Cargo.toml             # Rust dependencies
└── README.md              # This file
```

### Module Organization

#### 📊 `constants/` - Mathematical Constants

- **`fft_constants.rs`**: Precomputed FFT roots and related constants
- **`ntt_constants.rs`**: Precomputed NTT roots and modular arithmetic constants

#### 🔐 `crypto/` - Cryptographic Operations

- **`falcon.rs`**: Main Falcon signature scheme implementation
- **`ntrugen.rs`**: NTRU-based key pair generation algorithms
- **`encoding.rs`**: Signature compression and decompression

#### 🧮 `math/` - Mathematical Operations

- **`fft.rs`**: Fast Fourier transform over real numbers
- **`ntt.rs`**: Number theoretic transform over finite fields
- **`ffsampling.rs`**: Fast Fourier sampling - the heart of Falcon
- **`samplerz.rs`**: Gaussian sampling over integers

#### 🛠️ `utils/` - Utility Functions

- **`common.rs`**: Common functions, constants (Q = 12289), polynomial operations
- **`rng.rs`**: ChaCha20-based cryptographically secure random number generator

## ⚡ Performance

### Current Performance Metrics

Current performance results for Falcon-Rust (on Apple M1 Pro):

- **n=64**: FFT ~0.1-0.5ms, NTT ~0.1-0.5ms
- **n=128**: FFT ~0.2-1.0ms, NTT ~0.2-1.0ms
- **n=256**: FFT ~0.5-2.0ms, NTT ~0.5-2.0ms

### Dependencies

```toml
[dependencies]
num-complex = "0.4.6"  # Complex number arithmetic
rand = "0.9.1"         # Random number generation
```

## 🧪 Tests

### Test Categories

1. **Basic Operation Tests**: Addition, subtraction, modular arithmetic
2. **FFT/IFFT Tests**: Transform correctness and roundtrip
3. **NTT/INTT Tests**: Number theoretic transform correctness
4. **Performance Tests**: Timing measurements and benchmarks

### Test Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_fft_basic

# Run performance tests (optimized)
cargo test --release test_battery

# Run with detailed output
cargo test -- --nocapture
```

## 🔧 Development

### Development Environment Setup

```bash
# Clone the project
git clone https://github.com/your-username/falcon-rust.git
cd falcon-rust

# Build in development mode
cargo build

# Format code
cargo fmt

# Run linting
cargo clippy

# Run tests
cargo test
```

### Code Standards

- **Rust Edition**: 2024
- **Formatting**: Use `cargo fmt`
- **Linting**: Resolve `cargo clippy` warnings
- **Testing**: Add tests for new features
- **Documentation**: Add documentation comments for functions

### Commit Guidelines

We use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
feat: add new FFT optimization
fix: correct NTT modular arithmetic overflow
docs: update installation instructions
test: add comprehensive FFT test suite
refactor: improve polynomial multiplication performance
perf: optimize FFT algorithm
```

## 🤝 Contributing

We welcome contributions to improve Falcon-Rust!

### Ways to Contribute

1. **Opening Issues**: Bug reports, feature requests, or questions
2. **Pull Requests**: Bug fixes or new feature implementations
3. **Documentation**: README, code comments, or examples

### Development Process

1. **Fork** the repository and clone locally
2. **Create a new branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes** and test them
4. **Run tests**: `cargo test`
5. **Submit a pull request**

### Priority Development Areas

- [ ] Complete Falcon signature scheme implementation
- [ ] Key generation and management
- [ ] Signature creation and verification
- [ ] Performance optimizations
- [ ] Security audit
- [ ] Benchmark suite
- [ ] API documentation

## 🔒 Security

This implementation is for **educational and research purposes**. It is not intended for production use and has not undergone formal security audits.

For production-ready, official implementation, please visit [https://falcon-sign.info/](https://falcon-sign.info/).

## 🙏 Acknowledgments

Special thanks to:

- **Thomas Prest** - Original Python implementation
- **Falcon team** - Specification and reference implementation
- **Rust community** - Great tools and libraries

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Falcon Official Website](https://falcon-sign.info/)
- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [Rust Programming Language](https://www.rust-lang.org/)

---

**Note**: This is an educational implementation. For production use, please refer to the official Falcon implementation at [https://falcon-sign.info/](https://falcon-sign.info/).
