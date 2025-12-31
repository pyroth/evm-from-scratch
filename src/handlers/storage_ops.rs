use crate::error::Result;
use crate::stack::Stack;
use crate::storage::Storage;

pub fn handle_sload(storage: &Storage, stack: &mut Stack) -> Result<()> {
    let key = stack.pop()?;
    let value = storage.read(&key);
    stack.push(value)?;
    Ok(())
}

pub fn handle_sstore(storage: &mut Storage, stack: &mut Stack) -> Result<()> {
    let key = stack.pop()?;
    let value = stack.pop()?;
    storage.write(key, value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::U256;

    #[test]
    fn test_sstore_sload() {
        let mut storage = Storage::new();
        let mut stack = Stack::new();

        stack.push(U256::from(100)).unwrap();
        stack.push(U256::from(1)).unwrap();
        handle_sstore(&mut storage, &mut stack).unwrap();

        stack.push(U256::from(1)).unwrap();
        handle_sload(&storage, &mut stack).unwrap();

        assert_eq!(stack.top().unwrap(), U256::from(100));
    }
}
