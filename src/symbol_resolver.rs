use addr2line::Context;
use object::{Object, ObjectSection};
use std::fs;

// Resolves function names and source locations from addresses using debug symbols.
pub fn resolve_symbols(binary_path: &str, addresses: &[u64]) {
    let binary_data = fs::read(binary_path).expect("Failed to read binary file");
    let object_file = object::File::parse(&*binary_data).expect("Failed to parse binary file");
    let context = Context::new(&object_file).expect("Failed to create debug context");

    for &addr in addresses {
        match context.find_location(addr) {
            Ok(Some((function, location))) => {
                println!(
                    "Address: 0x{:x} -> Function: {} @ {:?}",
                    addr,
                    function.name().unwrap_or("unknown"),
                    location
                );
            }
            _ => {
                println!("Address: 0x{:x} -> No symbol found", addr);
            }
        }
    }
}
