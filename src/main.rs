mod analyser;
mod parser;

fn main() {
    let stack_dump_file = "stack_dump.txt";

    match parser::parse_stack_dump(stack_dump_file) {
        Ok(stack_trace) => {
            println!("🔍 Analyzing stack trace...");
            let fault_type = analyser::analyze_stack_trace(&stack_trace);
            println!("\nCrash Analysis Result: {:?}", fault_type);
        }
        Err(err) => eprintln!("❌ Error reading stack dump: {}", err),
    }
}

