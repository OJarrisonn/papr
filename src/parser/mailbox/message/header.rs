use std::fmt::Display;

use chrono::{DateTime, Utc};
use regex::Regex;

const PERSON_REGEX: &str = r"(?P<name>\w+(\s\w+)*)\s*<(?P<email>[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)>";
const PERSON_NO_NAME_REGEX: &str = r"<(?P<email>[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)>";
const PERSON_AT_REGEX: &str = r#"(?P<name>\w+(\s\w+)*)\s*at\s*<"(?P<email>[a-zA-Z0-9_.+-]+ at [a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)">"#;
const PERSON_AT_NO_NAME_REGEX: &str = r#""(?P<email>[a-zA-Z0-9_.+-]+ at [a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)""#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Header<'input> {
    From(Person<'input>),
    Date(DateTime<Utc>),
    Author(Person<'input>),
    Subject(&'input str),
    Other(&'input str, &'input str),
}

impl<'input> TryFrom<(&'input str, &'input str)> for Header<'input> {
    type Error = String;

    fn try_from(value: (&'input str, &'input str)) -> Result<Self, Self::Error> {
        let (key, value) = value;
        let lkey = key.to_lowercase();

        match lkey.as_str() {
            "from" => Ok(Header::From(value.try_into()?)),
            "date" => Ok(
                Header::Date(
                    DateTime::parse_from_rfc2822(value)
                        .map_err(|e| e.to_string())
                        .map(|dt| dt.to_utc())?
                )
            ),
            "author" => Ok(Header::Author(value.try_into()?)),
            "subject" => Ok(Header::Subject(value)),
            _ => Ok(Header::Other(key, value)),
        }
    }
}

impl Display for Header<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Header::From(person) => write!(f, "From: {}", person),
            Header::Date(date) => write!(f, "Date: {}", date.to_rfc2822()),
            Header::Author(person) => write!(f, "Author: {}", person),
            Header::Subject(subject) => write!(f, "Subject: {}", subject),
            Header::Other(key, value) => write!(f, "{}: {}", key, value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person<'input> {
    pub name: Option<&'input str>,
    pub email: &'input str,
}

impl<'input> TryFrom<&'input str> for Person<'input> {
    type Error = String;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let re_person = Regex::new(PERSON_REGEX).unwrap();
        let re_person_no_name = Regex::new(PERSON_NO_NAME_REGEX).unwrap();
        let re_person_at = Regex::new(PERSON_AT_REGEX).unwrap();
        let re_person_at_no_name = Regex::new(PERSON_AT_NO_NAME_REGEX).unwrap();

        let captures = re_person.captures(value)
            .or_else(|| re_person_no_name.captures(value))
            .or_else(|| re_person_at.captures(value))
            .or_else(|| re_person_at_no_name.captures(value))
            .ok_or("Invalid person")?;

        let name = captures.name("name").map(|name| name.as_str());
        let email = captures.name("email").ok_or("Invalid email")?.as_str();

        Ok(Person { name, email })
    }
}

impl Display for Person<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.name {
            write!(f, "{} <{}>", name, self.email)
        } else {
            write!(f, "<{}>", self.email)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email<'input> {
    pub user: &'input str,
    pub domain: &'input str,
}

impl<'input> TryFrom<&'input str> for Email<'input> {
    type Error = String;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = if value.contains('@') {
            value.split('@').collect()
        } else {
            value.split(" at ").collect()
        };

        if parts.len() != 2 {
            return Err(format!("Invalid email {}", value));
        }

        Ok(Email { user: parts[0], domain: parts[1] })
    }
}

impl Display for Email<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.user, self.domain)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_person_header() {
        let header = Header::try_from(("From", "Foo Bar <foo@baz.com>"));
        dbg!(&header);
    }

    #[test]
    fn parse_date_header() {
        let header = Header::try_from(("Date", "Wed, 08 Jun 2022 12:00:01 -0300"));
        assert!(header.is_ok());

        let header = header.unwrap();
        assert_eq!(header, Header::Date(DateTime::parse_from_rfc2822("Wed, 08 Jun 2022 12:00:01 -0300").unwrap().to_utc()));
    }

    #[test]
    fn parse_person() {
        let person = Person::try_from("Foo Bar <foo.bar@bar.com>");
        assert!(person.is_ok());

        let person = person.unwrap();
        assert_eq!(person.name, Some("Foo Bar"));
        assert_eq!(person.email, "foo.bar@bar.com");
    }
}