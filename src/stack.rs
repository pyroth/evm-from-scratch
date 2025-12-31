use crate::error::{EvmError, Result};
use primitive_types::U256;

const MAX_STACK_SIZE: usize = 1024;

#[derive(Debug, Default)]
pub struct Stack {
    data: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(MAX_STACK_SIZE),
        }
    }

    pub fn push(&mut self, value: U256) -> Result<()> {
        if self.data.len() >= MAX_STACK_SIZE {
            return Err(EvmError::StackOverflow);
        }
        self.data.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<U256> {
        self.data.pop().ok_or(EvmError::StackUnderflow)
    }

    pub fn top(&self) -> Result<U256> {
        self.data.last().copied().ok_or(EvmError::StackUnderflow)
    }

    pub fn at(&self, index: usize) -> Result<U256> {
        self.data
            .get(index)
            .copied()
            .ok_or(EvmError::StackUnderflow)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn swap(&mut self, index_a: usize, index_b: usize) -> Result<()> {
        let len = self.data.len();
        if index_a >= len || index_b >= len {
            return Err(EvmError::StackUnderflow);
        }
        self.data.swap(index_a, index_b);
        Ok(())
    }

    pub fn peek(&self, depth: usize) -> Result<U256> {
        if depth >= self.data.len() {
            return Err(EvmError::StackUnderflow);
        }
        Ok(self.data[self.data.len() - 1 - depth])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        stack.push(U256::from(42)).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(42));
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::new();
        for i in 0..MAX_STACK_SIZE {
            stack.push(U256::from(i)).unwrap();
        }
        assert_eq!(stack.push(U256::from(0)), Err(EvmError::StackOverflow));
    }

    #[test]
    fn test_stack_underflow() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), Err(EvmError::StackUnderflow));
    }
}
