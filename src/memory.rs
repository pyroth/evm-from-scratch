use primitive_types::U256;

#[derive(Debug, Default)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn expand_to(&mut self, offset: usize) {
        if offset >= self.data.len() {
            let new_size = ((offset + 32) / 32) * 32;
            self.data.resize(new_size, 0);
        }
    }

    pub fn write(&mut self, offset: U256, value: u8) {
        let offset = offset.as_usize();
        self.expand_to(offset);
        self.data[offset] = value;
    }

    pub fn read(&mut self, offset: U256) -> u8 {
        let offset = offset.as_usize();
        if offset >= self.data.len() {
            self.expand_to(offset);
        }
        self.data[offset]
    }

    pub fn write_bytes(&mut self, offset: U256, data: &[u8]) {
        let offset = offset.as_usize();
        self.expand_to(offset + data.len());
        self.data[offset..offset + data.len()].copy_from_slice(data);
    }

    pub fn read_bytes(&mut self, offset: U256, size: usize) -> Vec<u8> {
        let offset = offset.as_usize();
        self.expand_to(offset + size);
        self.data[offset..offset + size].to_vec()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_write_read() {
        let mut memory = Memory::new();
        memory.write(U256::from(0), 0x42);
        assert_eq!(memory.read(U256::from(0)), 0x42);
    }

    #[test]
    fn test_memory_expansion() {
        let mut memory = Memory::new();
        memory.write(U256::from(100), 0xff);
        assert!(memory.size() >= 100);
    }
}
