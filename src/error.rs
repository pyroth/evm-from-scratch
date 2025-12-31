use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EvmError {
    #[error("Stack overflow: maximum stack size is 1024")]
    StackOverflow,

    #[error("Stack underflow: not enough values on stack")]
    StackUnderflow,

    #[error("Invalid opcode: 0x{0:02x}")]
    InvalidOpcode(u8),

    #[error("Invalid PUSH: not enough bytes left in bytecode")]
    InvalidPush,

    #[error("Invalid DUP: not enough values on stack")]
    InvalidDup,

    #[error("Invalid SWAP: not enough values on stack")]
    InvalidSwap,

    #[error("Memory access out of bounds")]
    MemoryOutOfBounds,

    #[error("Invalid hex string: {0}")]
    InvalidHex(String),
}

pub type Result<T> = std::result::Result<T, EvmError>;
