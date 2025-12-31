use std::collections::HashMap;
use primitive_types::U256;

#[derive(Debug, Default)]
pub struct Storage {
    data: HashMap<U256, U256>,
}

impl Storage {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn write(&mut self, key: U256, value: U256) {
        self.data.insert(key, value);
    }

    pub fn read(&self, key: &U256) -> U256 {
        self.data.get(key).copied().unwrap_or(U256::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_write_read() {
        let mut storage = Storage::new();
        storage.write(U256::from(1), U256::from(100));
        assert_eq!(storage.read(&U256::from(1)), U256::from(100));
    }

    #[test]
    fn test_storage_default_zero() {
        let storage = Storage::new();
        assert_eq!(storage.read(&U256::from(999)), U256::zero());
    }
}
