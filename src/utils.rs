fn find_messages(input: &str) -> Vec<usize> {
    let mut input = input;
    let mut messages = Vec::new();
    let mut start = 0;

    while let Some(next) = input.find("\nFrom ") {
        let next = next + 1;
        messages.push(start + next);
        input = &input[next..];
        start += next;
    }

    messages
}

pub fn capture_messages(input: &str) -> Vec<&str> {
    let mut messages = Vec::new();
    let mut start = 0;

    for next in find_messages(input) {
        messages.push(&input[start..next]);
        start = next;
    }

    messages.push(&input[start..]);

    messages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_messages_test() {
        let input = include_str!("samples/multi_foo_messages.mbx");
        let messages = capture_messages(input);
        assert!(messages.len() == 4);
    }

    #[test]
    fn find_messages_test() {
        let input = include_str!("parser/mailbox/samples/multi_patches.mbx");
        let messages = capture_messages(input);
        dbg!(messages);
    }
}
