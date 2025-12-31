use crate::error::{EvmError, Result};

pub fn hex_to_bytes(input: &str) -> Result<Vec<u8>> {
    let input = input.strip_prefix("0x").unwrap_or(input);
    
    if input.len() % 2 != 0 {
        return Err(EvmError::InvalidHex("odd length hex string".to_string()));
    }
    
    hex::decode(input).map_err(|e| EvmError::InvalidHex(e.to_string()))
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        assert_eq!(hex_to_bytes("0x6001").unwrap(), vec![0x60, 0x01]);
        assert_eq!(hex_to_bytes("6001").unwrap(), vec![0x60, 0x01]);
    }

    #[test]
    fn test_bytes_to_hex() {
        assert_eq!(bytes_to_hex(&[0x60, 0x01]), "0x6001");
    }
}
