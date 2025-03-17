use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum FaultType {
    PageDomainFault,
    SegmentationFault,
    NullPointerDereference,
    MemoryLeak,
    Unknown,
}

/// Extact faulting address from the stack trace
fn extract_fault_address(stack_trace: &str) -> Option<u64> {
    let re = Regex::new(r"Unhandled fault: page domain fault \(.*?\) at (0x[0-9a-fA-F]+)").unwrap();
    if let Some(captures) = re.captures(stack_trace) {
        if let Some(addr) = captures.get(1) {
            return u64::from_str_radix(addr.as_str().trim_start_matches("0x"), 16).ok();
        }
    }
    None
}

/// Detects memory access violations
pub fn analyze_stack_trace(stack_trace: &str) -> FaultType {
    if let Some(fault_address) = extract_fault_address(stack_trace) {
        println!(
            "🚨 Detected Unhandled Page Domain Fault at 0x{:x}",
            fault_address
        );

        // Detect segementation faults or null pointer issues
        if fault_address == 0x0 {
            println!("💀 Segmentation Fault: Attempted to access NULL memory.");
            return FaultType::SegmentationFault;
        }

        // Detect use-after-free based on register memory classifcation
        if stack_trace.contains("non-paged memory") {
            println!("🛑 Use-after-free detected: Accessed freed memory.");
            return FaultType::MemoryLeak;
        }

        return FaultType::PageDomainFault;
    }

    println!("✅ No critical memory issues detected.");
    FaultType::Unknown
}

