/// Returns the next line in the input string
pub fn next_line(input: &str) -> Option<&str> {
    input.find('\n').map(|i| &input[i + 1..])
}

/// Returns the next non-empty line in the input string
pub fn next_non_empty_line(input: &str) -> Option<&str> {
    let mut input = input;
    while let Some(line) = next_line(input) {
        if line.trim().is_empty() {
            input = line;
        } else {
            return Some(line);
        }
    }
    None
}
