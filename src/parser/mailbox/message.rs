use std::fmt::Display;

use chrono::{DateTime, Utc};
use header::Header;

pub mod header;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents the mailer line of a message it is used to identify where a new message starts
/// in a mbox file
/// 
/// # Example
/// - `From git@z Thu Jan  1 00:00:00 1970` the default git mailer line
pub struct Mailer<'input> {
    pub daemon: &'input str,
    pub date: DateTime<Utc>
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a message in a mbox file 
/// 
/// Is composed of a mailer line, a list of headers and a body
pub struct Message<'input> {
    pub mailer: Option<Mailer<'input>>,
    pub headers: Vec<Header<'input>>,
    pub body: &'input str
}

impl<'input> TryFrom<&'input str> for Message<'input> {
    type Error = String;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let mut headers = Vec::new();
        let mut body = "";

        if lines.clone().count() == 0 {
            return Err("Empty message".to_string());
        }

        let mailer = lines.next().map(|line| {
            Mailer::try_from(line)
        }).transpose()?;

        for line in lines {
            if line.trim().is_empty() {
                let break_point = value.find("\n\n").unwrap_or(value.len());
                body = &value[break_point..];
                break;
            }

            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap().trim();
            let value = parts.next().unwrap_or("").trim();
            headers.push((key, value).try_into()?);
        }

        Ok(Message { mailer, headers, body })
    }
}

impl Display for Message<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for header in &self.headers {
            write!(f, "{}\n", header)?;
        }

        write!(f, "\n{}", self.body)
    }
}

impl<'input> TryFrom<&'input str> for Mailer<'input> {
    type Error = String;
    
    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let parts = value.split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>();

        if parts.len() < 7 {
            return Err(format!("Invalid mailer line: {}", value));
        }

        if parts[0] != "From" {
            return Err(format!("Invalid mailer line: {}. Should start with `From `", value));
        }

        let daemon = parts[1];

        // FIXME: Timezone is hardcoded to +0000
        let date = format!("{} {} {} {} {} +0000", parts[2], parts[3], parts[4], parts[5], parts[6]);
        let date = DateTime::parse_from_str(&date, "%a %b %e %H:%M:%S %Y %z")
            .map(|d| d.with_timezone(&Utc))
            .map_err(|e| format!("Invalid date: {}", e))?;

        Ok(Mailer { daemon, date })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mailer_try_from_empty() {
        let mailer = Mailer::try_from("");
        assert!(mailer.is_err());
    }

    #[test]
    fn test_mailer_try_from_git_date() {
        let mailer = Mailer::try_from("From git@z Thu Jan  1 00:00:00 1970");
        let mailer = dbg!(mailer);
        assert!(mailer.is_ok());
    }

    #[test]
    fn test_mailer_try_from_git_date_invalid() {
        let mailer = Mailer::try_from("From git@z Thu Jan  1 00:00:00");
        assert!(mailer.is_err());
    }

    #[test]
    fn test_message_try_from_empty() {
        let message = Message::try_from("");
        assert!(message.is_err());
    }

    #[test]
    fn test_message_try_from_git() {
        let message = Message::try_from(include_str!("samples/single_patch.mbx"));
        let message = dbg!(message);
        assert!(message.is_ok());
        let message = message.unwrap();
        assert_eq!(message.mailer.unwrap().daemon, "git@z");

        assert_eq!(message.headers.len(), 7);
        assert_eq!(message.headers[0], ("Subject", "[PATCH v1 1/10] patch-tree: foo message").try_into().unwrap());
        assert_eq!(message.headers[1], ("From", "John Doe <\"john.doe at email.com\">").try_into().unwrap());
        assert_eq!(message.headers[2], ("Date", "Fri, 08 Jun 2022 12:00:01 -0300").try_into().unwrap());
        assert_eq!(message.headers[3], ("Message-Id", "<20220608-john-doe@email.com>").try_into().unwrap());
        assert_eq!(message.headers[4], ("MIME-Version", "1.0").try_into().unwrap());
        assert_eq!(message.headers[5], ("Content-Type", "text/plain; charset=\"utf-8\"").try_into().unwrap());
        assert_eq!(message.headers[6], ("Content-Transfer-Encoding", "7bit").try_into().unwrap());

        let break_point = include_str!("samples/single_patch.mbx").find("\n\n").unwrap_or(include_str!("samples/single_patch.mbx").len());
        assert_eq!(message.body, &include_str!("samples/single_patch.mbx")[break_point..]);
    }
}