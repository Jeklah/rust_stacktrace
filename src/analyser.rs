use std::collections::HashSet;

// Analyses stack addresses for potential buffer overflow indicators.
pub fn analyse_stack_overflow(addresses: &[u64]) {
    let mut seen_addresses = HashSet::new();
    let mut suspicious_addresses = Vec::new();

    for &addr in addresses {
        if seen_addresses.contains(&addr) {
            // If an address appears multiple times, it may indicate a stack corruption issue.
            suspicious_addresses.push(addr);
        }
        seen_addresses.insert(addr);
    }

    if !suspicious_addresses.is_empty() {
        println!("Potential stack overflow detected at the following addresses:");
        for addr in suspicious_addresses {
            println!(" - 0x{:x}", addr);
        }
    } else {
        println!("No obvious buffer overflow detected");
    }
}
