pub mod arithmetic;
pub mod stack_ops;
pub mod storage_ops;

pub use arithmetic::handle_arithmetic;
pub use stack_ops::{handle_dup, handle_push, handle_swap};
pub use storage_ops::{handle_sload, handle_sstore};
