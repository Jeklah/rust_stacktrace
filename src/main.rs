mod parser;
mod symbol_resolver;

fn main() {
    let stack_dump_file = "stack_dump.txt";
    let binary_file = "my_program";

    match parser::parse_stack_dump(stack_dump_file) {
        Ok(addresses) => {
            println!("Resolved stack addresses:");
            symbol_resolve::resolve_symbols(binary_file, &addresses);
        }
        Err(err) => {
            eprintln!("Error reading stack dump: {}", err);
        }
    }
}
