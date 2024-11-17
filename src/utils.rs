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

/// Given a mailbox file, returns the next message in the file
/// 
/// It will start looking from the `start` index in the input string
/// for the message delimiter pattern `--\n<number>\n\nFrom `. and 
/// return the index of the start of the message (the index of `F`).
pub fn find_next_message(input: &str, start: usize) -> Option<usize> {
    let input = &input[start..];
    let mut lines = input.lines();
    let mut stage = 0;
    let mut index = start;

    while let Some(line) = lines.next() {
        match stage {
            0 => {
                if line.trim_end() == "--" {
                    stage = 1;
                }
            },
            1 => {
                if line.chars().all(|c| c.is_digit(10) || c == '.') {
                    stage = 2;
                } else {
                    stage = 0;
                }
            },
            2 => {
                if line.is_empty() {
                    stage = 3;
                } else {
                    stage = 0;
                }
            },
            3 => {
                if !line.starts_with("From ") && !line.trim().is_empty() {
                    stage = 0;
                } else if line.starts_with("From ") {
                    return Some(index);
                }
            },
            _ => unreachable!()
        }
        index += line.len() + 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_messages() {
        let input = include_str!("../tests/samples/multi_foo_messages.mbx");

        let first_message = find_next_message(input, 0);
        assert!(first_message.is_some());
        assert!(input[first_message.unwrap()..].starts_with("From barbarbar"));
    
        let second_message = find_next_message(input, first_message.unwrap());
        assert!(second_message.is_some());
        assert!(&input[second_message.unwrap()..].starts_with("From foobar"));
    }
}