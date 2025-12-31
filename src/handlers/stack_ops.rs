use crate::error::{EvmError, Result};
use crate::opcodes;
use crate::stack::Stack;
use primitive_types::U256;

pub fn handle_push(opcode: u8, stack: &mut Stack, bytecode: &[u8], pc: &mut usize) -> Result<()> {
    let bytes_to_push = (opcode - opcodes::PUSH0) as usize;

    if bytes_to_push == 0 {
        stack.push(U256::zero())?;
        return Ok(());
    }

    if *pc + bytes_to_push > bytecode.len() {
        return Err(EvmError::InvalidPush);
    }

    let mut value = U256::zero();
    for i in 0..bytes_to_push {
        value = (value << 8) | U256::from(bytecode[*pc + i]);
    }

    *pc += bytes_to_push;
    stack.push(value)?;
    Ok(())
}

pub fn handle_dup(opcode: u8, stack: &mut Stack) -> Result<()> {
    let dup_index = (opcode - opcodes::DUP1 + 1) as usize;
    let size = stack.len();

    if size < dup_index {
        return Err(EvmError::InvalidDup);
    }

    let value = stack.at(size - dup_index)?;
    stack.push(value)?;
    Ok(())
}

pub fn handle_swap(opcode: u8, stack: &mut Stack) -> Result<()> {
    let swap_index = (opcode - opcodes::SWAP1 + 1) as usize;
    let size = stack.len();

    if size <= swap_index {
        return Err(EvmError::InvalidSwap);
    }

    stack.swap(size - 1, size - 1 - swap_index)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push1() {
        let mut stack = Stack::new();
        let bytecode = vec![0x60, 0x42];
        let mut pc = 1usize;
        handle_push(0x60, &mut stack, &bytecode, &mut pc).unwrap();
        assert_eq!(stack.top().unwrap(), U256::from(0x42));
        assert_eq!(pc, 2);
    }

    #[test]
    fn test_push0() {
        let mut stack = Stack::new();
        let bytecode = vec![0x5f];
        let mut pc = 1usize;
        handle_push(0x5f, &mut stack, &bytecode, &mut pc).unwrap();
        assert_eq!(stack.top().unwrap(), U256::zero());
    }

    #[test]
    fn test_dup1() {
        let mut stack = Stack::new();
        stack.push(U256::from(42)).unwrap();
        handle_dup(0x80, &mut stack).unwrap();
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.top().unwrap(), U256::from(42));
    }

    #[test]
    fn test_swap1() {
        let mut stack = Stack::new();
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        handle_swap(0x90, &mut stack).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(1));
        assert_eq!(stack.pop().unwrap(), U256::from(2));
    }
}
