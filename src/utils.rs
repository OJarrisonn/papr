use std::fmt::Display;

use color_print::cformat;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "rgb")]
    RGB(u8, u8, u8),
    #[serde(rename = "term")]
    Term(TermColor),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TermColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    #[default]
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn paint<T: Display>(&self, value: T) -> String {
        // TODO: Implement color formatting
        cformat!("{}", value)
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Term(TermColor::default())
    }
}

fn find_messages(input: &str) -> Vec<usize> {
    let mut input = input;
    let mut messages = Vec::from([0]);
    let mut start = 0;

    while let Some(next) = input.find("\nFrom ") {
        let next = next + 1;
        messages.push(start + next);
        input = &input[next..];
        start += next;
    }

    messages
}

pub fn capture_messages<'input>(input: &'input str) -> Vec<&'input str> {
    let starts = find_messages(input);
    let mut ends = starts.clone();
    ends.push(input.len());
    ends.remove(0);

    let ranges = starts.iter().zip(ends.iter());

    ranges
        .par_bridge()
        .map(|(start, end)| &input[*start..*end])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_messages_test() {
        let input = include_str!("samples/multi_foo_messages.mbx");
        let messages = find_messages(input);
        assert!(messages.len() == 4);
    }

    #[test]
    fn capture_messages_test() {
        let input = include_str!("samples/multi_foo_messages.mbx");
        let messages = capture_messages(input);
        assert!(messages.len() == 4);
    }

    #[test]
    fn capture_multi_patches_messages_test() {
        let input = include_str!("mailbox/samples/multi_patches.mbx");
        let messages = capture_messages(input);
        dbg!(messages);
    }

    #[test]
    fn test_color() {
        let color = Color::RGB(0, 0, 0);
        let serialized = toml::to_string(&color).unwrap();
        assert_eq!(serialized, r#"{"RGB":[0,0,0]}"#);

        let deserialized: Color = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized, color);
    }
}
