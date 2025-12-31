use evm::Evm;
use evm::utils::hex_to_bytes;

fn main() {
    let mut evm = Evm::new();
    
    // Example: PUSH1 0x05, PUSH1 0x03, ADD (5 + 3 = 8)
    let bytecode = hex_to_bytes("0x6005600301").expect("Invalid bytecode");
    
    evm.execute(&bytecode).expect("Execution failed");
    
    match evm.stack_top() {
        Ok(result) => println!("Stack top: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
