use std::fs;

pub fn parse_stack_dump(file_path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

