use std::fmt::Display;

use message::Message;

use crate::utils;

pub mod message;

#[derive(Debug, PartialEq, Eq)]
pub struct Mailbox<'input> {
    pub messages: Vec<Message<'input>>,
}

impl<'input> TryFrom<&'input str> for Mailbox<'input> {
    type Error = String;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        let messages = utils::capture_messages(input)
            .iter()
            .map(|message| Message::try_from(*message))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Mailbox { messages })
    }
}

impl Display for Mailbox<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for message in &self.messages {
            write!(f, "{}\n\n", message)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mailbox() {
        let input = include_str!("mailbox/samples/multi_patches.mbx");
        let mailbox = Mailbox::try_from(input);
        assert!(mailbox.is_ok());
        let mailbox = mailbox.unwrap();

        dbg!(&mailbox);
        assert!(mailbox.messages.len() == 3);
    }
}