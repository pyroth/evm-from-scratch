# EVM From Scratch

A minimal Ethereum Virtual Machine implementation in Rust for educational purposes.

## Features

- **Stack-based execution** — 256-bit word size with 1024 element limit
- **Arithmetic** — ADD, MUL, SUB, DIV, SDIV, MOD, SMOD, ADDMOD, MULMOD, EXP, SIGNEXTEND
- **Comparison** — LT, GT, SLT, SGT, EQ, ISZERO
- **Bitwise** — AND, OR, XOR, NOT, BYTE, SHL, SHR, SAR
- **Stack ops** — PUSH0-PUSH32, DUP1-DUP16, SWAP1-SWAP16, POP
- **Storage** — SLOAD, SSTORE

## Usage

```rust
use evm::{Evm, U256};

fn main() {
    let mut evm = Evm::new();
    
    // PUSH1 0x01, PUSH1 0x02, ADD
    let bytecode = hex::decode("6001600201").unwrap();
    evm.execute(&bytecode).unwrap();
    
    assert_eq!(evm.stack_top().unwrap(), U256::from(3));
}
```

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
