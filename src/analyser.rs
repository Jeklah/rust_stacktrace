use std::collections::{HashMap, HashSet};

/// Known allocation functions (C/C++ interop or Rust allocations)
const ALLOCATION_FUNCS: &[&str] = &["malloc", "calloc", "realloc", "new", "operator new"];

/// Known deallocation functions
const DEALLOCATION_FUNCS: &[&str] = &["free", "delete", "operator delete"];

/// Known memory manipulation functions (potential invalid accesses)
const MEMORY_ACCESS_FUNCS: &[&str] = &["memcpy", "memmove", "memset", "strcpy", "strncpy"];


/// Represents a stack trace with memory addresses.
#[derive(Debug, Default)]
pub enum FaultType {
    MemoryLeak,
    PageFault,
    SegmentationFault,
    Unknown,
}

/// Analyzee stack trace for memory leaks,  and unhandled exceptions and page faults.
pub fn analyze_stack_trace(function_calls: &[String], addresses: &[u64]) -> FaultType {
    let mut allocations = HashMap::new();
    let mut deallocations = HashSet::new();
    let mut last_function = function_calls.last().unwrap_or(&"Unknown".to_string());
    

    // Track memory allocations and deallocations
    for func in function_calls {
        if ALLOCATION_FUNCS.iter().any(|&alloc| func.contains(alloc)) {
            *allocations.entry(func.clone()).or_insert(0) += 1;
        }
        if DEALLOCATION_FUNCS.iter().any(|&dealloc| func.contains(dealloc)) {
            deallocations.insert(func.clone());
        }
    }

    // Check for memory leaks
    let mut leaks_found = false;
    for (func, count) in allocations_sites {
        if !deallocations.contains(func){
            println!("⚠️  Memory leak detected: {} ({} allocations)", func, count);
            leaks_found = true;
        }
    }

    // Check for unhandled page faults
    if MEMORY_ACCESS_FUNCS.iter().any(|&faulty_func| last_function.contains(faulty_func)) {
        println!("🚨 Page Domain Fault detected: {}", last_function);
        return FaultType::PageFault;
    }

    // Detect Segementation fault (null pointer or access violation)
    if let Some(&last_addr) = addresses.last() {
        if last_addr == 0x0 || last_addr > 0xffff_ffff_ffff {
            println!("💀 Segmentation Fault detected: {:x}", last_addr);
            return FaultType::SegmentationFault;
        }
    }

    if leaks_found {
        return FaultType::MemoryLeak;
    }

    println!("✅ No issues detected");
    FaultType::Unknown
}
