use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Parses a stack dump file and extracts memory addresses.
// Assumes the stack dump contains one address per line in hexadecimal format.

pub fn parse_stack_dump(filename: &str) -> io::Result<Vec<u64>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut addresses = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Ok(addr) = u64::from_str_radix(line.trim(), 16) {
            addresses.push(addr);
        } else {
            eprintln!("Warning: Skipping invalid address: {}", line);
        }
    }

    Ok(addresses)
}
