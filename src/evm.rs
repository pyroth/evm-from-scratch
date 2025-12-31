use crate::error::{EvmError, Result};
use crate::handlers::{
    handle_arithmetic, handle_dup, handle_push, handle_sload, handle_sstore, handle_swap,
};
use crate::memory::Memory;
use crate::opcodes::{self, is_dup, is_push, is_swap};
use crate::stack::Stack;
use crate::storage::Storage;
use primitive_types::U256;

#[derive(Debug, Default)]
pub struct Evm {
    stack: Stack,
    storage: Storage,
    memory: Memory,
    pc: usize,
    running: bool,
}

impl Evm {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            storage: Storage::new(),
            memory: Memory::new(),
            pc: 0,
            running: true,
        }
    }

    pub fn execute(&mut self, bytecode: &[u8]) -> Result<()> {
        self.pc = 0;
        self.running = true;

        while self.running && self.pc < bytecode.len() {
            let opcode = bytecode[self.pc];
            self.pc += 1;

            self.execute_opcode(opcode, bytecode)?;
        }

        Ok(())
    }

    fn execute_opcode(&mut self, opcode: u8, bytecode: &[u8]) -> Result<()> {
        match opcode {
            opcodes::STOP => {
                self.running = false;
            }

            // Arithmetic & Comparison & Bitwise Operations
            opcodes::ADD
            | opcodes::MUL
            | opcodes::SUB
            | opcodes::DIV
            | opcodes::SDIV
            | opcodes::MOD
            | opcodes::SMOD
            | opcodes::ADDMOD
            | opcodes::MULMOD
            | opcodes::EXP
            | opcodes::SIGNEXTEND
            | opcodes::LT
            | opcodes::GT
            | opcodes::SLT
            | opcodes::SGT
            | opcodes::EQ
            | opcodes::ISZERO
            | opcodes::AND
            | opcodes::OR
            | opcodes::XOR
            | opcodes::NOT
            | opcodes::BYTE
            | opcodes::SHL
            | opcodes::SHR
            | opcodes::SAR => {
                handle_arithmetic(opcode, &mut self.stack)?;
            }

            // Storage Operations
            opcodes::SLOAD => {
                handle_sload(&self.storage, &mut self.stack)?;
            }
            opcodes::SSTORE => {
                handle_sstore(&mut self.storage, &mut self.stack)?;
            }

            // Stack Manipulation
            opcodes::POP => {
                self.stack.pop()?;
            }
            _ if is_push(opcode) => {
                handle_push(opcode, &mut self.stack, bytecode, &mut self.pc)?;
            }
            _ if is_dup(opcode) => {
                handle_dup(opcode, &mut self.stack)?;
            }
            _ if is_swap(opcode) => {
                handle_swap(opcode, &mut self.stack)?;
            }

            _ => return Err(EvmError::InvalidOpcode(opcode)),
        }

        Ok(())
    }

    pub fn stack_top(&self) -> Result<U256> {
        self.stack.top()
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::hex_to_bytes;

    #[test]
    fn test_push1_stop() {
        let mut evm = Evm::new();
        let bytecode = hex_to_bytes("0x6042").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(0x42));
    }

    #[test]
    fn test_add() {
        let mut evm = Evm::new();
        // PUSH1 0x01, PUSH1 0x02, ADD
        let bytecode = hex_to_bytes("0x6001600201").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(3));
    }

    #[test]
    fn test_mul() {
        let mut evm = Evm::new();
        // PUSH1 0x03, PUSH1 0x04, MUL
        let bytecode = hex_to_bytes("0x6003600402").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(12));
    }

    #[test]
    fn test_sub() {
        let mut evm = Evm::new();
        // PUSH1 0x03, PUSH1 0x05, SUB
        let bytecode = hex_to_bytes("0x6003600503").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(2));
    }

    #[test]
    fn test_div() {
        let mut evm = Evm::new();
        // PUSH1 0x02, PUSH1 0x06, DIV
        let bytecode = hex_to_bytes("0x6002600604").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(3));
    }

    #[test]
    fn test_div_by_zero() {
        let mut evm = Evm::new();
        // PUSH1 0x00, PUSH1 0x06, DIV
        let bytecode = hex_to_bytes("0x6000600604").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::zero());
    }

    #[test]
    fn test_mod() {
        let mut evm = Evm::new();
        // PUSH1 0x03, PUSH1 0x0a, MOD
        let bytecode = hex_to_bytes("0x6003600a06").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(1));
    }

    #[test]
    fn test_dup1() {
        let mut evm = Evm::new();
        // PUSH1 0x42, DUP1
        let bytecode = hex_to_bytes("0x604280").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack.len(), 2);
        assert_eq!(evm.stack_top().unwrap(), U256::from(0x42));
    }

    #[test]
    fn test_swap1() {
        let mut evm = Evm::new();
        // PUSH1 0x01, PUSH1 0x02, SWAP1
        let bytecode = hex_to_bytes("0x6001600290").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(1));
    }

    #[test]
    fn test_pop() {
        let mut evm = Evm::new();
        // PUSH1 0x01, PUSH1 0x02, POP
        let bytecode = hex_to_bytes("0x6001600250").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack.len(), 1);
        assert_eq!(evm.stack_top().unwrap(), U256::from(1));
    }

    #[test]
    fn test_sstore_sload() {
        let mut evm = Evm::new();
        // PUSH1 0x64 (value=100), PUSH1 0x01 (key=1), SSTORE, PUSH1 0x01, SLOAD
        let bytecode = hex_to_bytes("0x6064600155600154").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(100));
    }

    #[test]
    fn test_iszero() {
        let mut evm = Evm::new();
        // PUSH1 0x00, ISZERO
        let bytecode = hex_to_bytes("0x600015").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::one());
    }

    #[test]
    fn test_lt() {
        let mut evm = Evm::new();
        // PUSH1 0x02, PUSH1 0x01, LT (1 < 2 = true)
        let bytecode = hex_to_bytes("0x6002600110").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::one());
    }

    #[test]
    fn test_gt() {
        let mut evm = Evm::new();
        // PUSH1 0x01, PUSH1 0x02, GT (2 > 1 = true)
        let bytecode = hex_to_bytes("0x6001600211").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::one());
    }

    #[test]
    fn test_eq() {
        let mut evm = Evm::new();
        // PUSH1 0x05, PUSH1 0x05, EQ
        let bytecode = hex_to_bytes("0x6005600514").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::one());
    }

    #[test]
    fn test_and() {
        let mut evm = Evm::new();
        // PUSH1 0x0f, PUSH1 0xff, AND
        let bytecode = hex_to_bytes("0x600f60ff16").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(0x0f));
    }

    #[test]
    fn test_or() {
        let mut evm = Evm::new();
        // PUSH1 0x0f, PUSH1 0xf0, OR
        let bytecode = hex_to_bytes("0x600f60f017").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(0xff));
    }

    #[test]
    fn test_xor() {
        let mut evm = Evm::new();
        // PUSH1 0xff, PUSH1 0xf0, XOR
        let bytecode = hex_to_bytes("0x60ff60f018").unwrap();
        evm.execute(&bytecode).unwrap();
        assert_eq!(evm.stack_top().unwrap(), U256::from(0x0f));
    }
}
