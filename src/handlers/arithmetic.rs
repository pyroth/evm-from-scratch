use crate::error::{EvmError, Result};
use crate::opcodes;
use crate::stack::Stack;
use primitive_types::U256;

pub fn handle_arithmetic(opcode: u8, stack: &mut Stack) -> Result<()> {
    match opcode {
        opcodes::ADD => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            stack.push(a.overflowing_add(b).0)?;
        }
        opcodes::MUL => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            stack.push(a.overflowing_mul(b).0)?;
        }
        opcodes::SUB => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            stack.push(a.overflowing_sub(b).0)?;
        }
        opcodes::DIV => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            let result = if b.is_zero() { U256::zero() } else { a / b };
            stack.push(result)?;
        }
        opcodes::SDIV => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            let result = if b.is_zero() {
                U256::zero()
            } else {
                signed_div(a, b)
            };
            stack.push(result)?;
        }
        opcodes::MOD => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            let result = if b.is_zero() { U256::zero() } else { a % b };
            stack.push(result)?;
        }
        opcodes::SMOD => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            let result = if b.is_zero() {
                U256::zero()
            } else {
                signed_mod(a, b)
            };
            stack.push(result)?;
        }
        opcodes::ADDMOD => {
            ensure_stack_size(stack, 3)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            let n = stack.pop()?;
            let result = if n.is_zero() {
                U256::zero()
            } else {
                addmod(a, b, n)
            };
            stack.push(result)?;
        }
        opcodes::MULMOD => {
            ensure_stack_size(stack, 3)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            let n = stack.pop()?;
            let result = if n.is_zero() {
                U256::zero()
            } else {
                mulmod(a, b, n)
            };
            stack.push(result)?;
        }
        opcodes::EXP => {
            ensure_stack_size(stack, 2)?;
            let base = stack.pop()?;
            let exponent = stack.pop()?;
            let result = exp_by_squaring(base, exponent);
            stack.push(result)?;
        }
        opcodes::SIGNEXTEND => {
            ensure_stack_size(stack, 2)?;
            let b = stack.pop()?;
            let x = stack.pop()?;
            let result = if b < U256::from(32) {
                let bit_index = (b.low_u32() as usize + 1) * 8 - 1;
                let mask = U256::one() << bit_index;
                if x & mask != U256::zero() {
                    x | (U256::MAX << bit_index)
                } else {
                    x & ((U256::one() << (bit_index + 1)) - 1)
                }
            } else {
                x
            };
            stack.push(result)?;
        }
        opcodes::LT => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            stack.push(if a < b { U256::one() } else { U256::zero() })?;
        }
        opcodes::GT => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            stack.push(if a > b { U256::one() } else { U256::zero() })?;
        }
        opcodes::SLT => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            let result = signed_lt(a, b);
            stack.push(if result { U256::one() } else { U256::zero() })?;
        }
        opcodes::SGT => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            let result = signed_lt(b, a);
            stack.push(if result { U256::one() } else { U256::zero() })?;
        }
        opcodes::EQ => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            stack.push(if a == b { U256::one() } else { U256::zero() })?;
        }
        opcodes::ISZERO => {
            ensure_stack_size(stack, 1)?;
            let a = stack.pop()?;
            stack.push(if a.is_zero() {
                U256::one()
            } else {
                U256::zero()
            })?;
        }
        opcodes::AND => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            stack.push(a & b)?;
        }
        opcodes::OR => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            stack.push(a | b)?;
        }
        opcodes::XOR => {
            ensure_stack_size(stack, 2)?;
            let a = stack.pop()?;
            let b = stack.pop()?;
            stack.push(a ^ b)?;
        }
        opcodes::NOT => {
            ensure_stack_size(stack, 1)?;
            let a = stack.pop()?;
            stack.push(!a)?;
        }
        opcodes::BYTE => {
            ensure_stack_size(stack, 2)?;
            let i = stack.pop()?;
            let x = stack.pop()?;
            let result = if i >= U256::from(32) {
                U256::zero()
            } else {
                let byte_idx = 31 - i.low_u32() as usize;
                let bytes: [u8; 32] = x.to_big_endian();
                U256::from(bytes[31 - byte_idx])
            };
            stack.push(result)?;
        }
        opcodes::SHL => {
            ensure_stack_size(stack, 2)?;
            let shift = stack.pop()?;
            let value = stack.pop()?;
            let result = if shift >= U256::from(256) {
                U256::zero()
            } else {
                value << shift.low_u32() as usize
            };
            stack.push(result)?;
        }
        opcodes::SHR => {
            ensure_stack_size(stack, 2)?;
            let shift = stack.pop()?;
            let value = stack.pop()?;
            let result = if shift >= U256::from(256) {
                U256::zero()
            } else {
                value >> shift.low_u32() as usize
            };
            stack.push(result)?;
        }
        opcodes::SAR => {
            ensure_stack_size(stack, 2)?;
            let shift = stack.pop()?;
            let value = stack.pop()?;
            let result = signed_shr(value, shift);
            stack.push(result)?;
        }
        _ => return Err(EvmError::InvalidOpcode(opcode)),
    }
    Ok(())
}

#[inline]
fn ensure_stack_size(stack: &Stack, required: usize) -> Result<()> {
    if stack.len() < required {
        return Err(EvmError::StackUnderflow);
    }
    Ok(())
}

fn exp_by_squaring(base: U256, exp: U256) -> U256 {
    if exp.is_zero() {
        return U256::one();
    }

    let mut result = U256::one();
    let mut base = base;
    let mut exp = exp;

    while !exp.is_zero() {
        if exp & U256::one() == U256::one() {
            result = result.overflowing_mul(base).0;
        }
        exp >>= 1;
        base = base.overflowing_mul(base).0;
    }
    result
}

fn is_negative(value: U256) -> bool {
    value.bit(255)
}

fn negate(value: U256) -> U256 {
    (!value).overflowing_add(U256::one()).0
}

fn signed_div(a: U256, b: U256) -> U256 {
    let a_neg = is_negative(a);
    let b_neg = is_negative(b);

    let a_abs = if a_neg { negate(a) } else { a };
    let b_abs = if b_neg { negate(b) } else { b };

    let result = a_abs / b_abs;

    if a_neg != b_neg {
        negate(result)
    } else {
        result
    }
}

fn signed_mod(a: U256, b: U256) -> U256 {
    let a_neg = is_negative(a);
    let b_neg = is_negative(b);

    let a_abs = if a_neg { negate(a) } else { a };
    let b_abs = if b_neg { negate(b) } else { b };

    let result = a_abs % b_abs;

    if a_neg { negate(result) } else { result }
}

fn signed_lt(a: U256, b: U256) -> bool {
    let a_neg = is_negative(a);
    let b_neg = is_negative(b);

    match (a_neg, b_neg) {
        (true, false) => true,
        (false, true) => false,
        _ => a < b,
    }
}

fn signed_shr(value: U256, shift: U256) -> U256 {
    if shift >= U256::from(256) {
        if is_negative(value) {
            U256::MAX
        } else {
            U256::zero()
        }
    } else {
        let shift = shift.low_u32() as usize;
        let result = value >> shift;
        if is_negative(value) {
            result | (U256::MAX << (256 - shift))
        } else {
            result
        }
    }
}

fn addmod(a: U256, b: U256, n: U256) -> U256 {
    let (sum, overflow) = a.overflowing_add(b);
    if overflow {
        let remainder = U256::MAX - n + 1;
        let adjusted = sum.overflowing_add(remainder).0;
        adjusted % n
    } else {
        sum % n
    }
}

fn mulmod(a: U256, b: U256, n: U256) -> U256 {
    if a.is_zero() || b.is_zero() {
        return U256::zero();
    }

    let mut result = U256::zero();
    let mut a = a % n;
    let mut b = b;

    while !b.is_zero() {
        if b & U256::one() == U256::one() {
            result = addmod(result, a, n);
        }
        a = addmod(a, a, n);
        b >>= 1;
    }
    result
}
