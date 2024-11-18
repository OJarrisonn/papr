use std::fmt::Display;

use chrono::{DateTime, Utc};
use color_print::{cformat, cwrite};
use regex::Regex;

const PERSON_REGEX: &str = r"(?P<name>\w+(\s\w+)*)\s*<(?P<email>[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)>";
const PERSON_NO_NAME_REGEX: &str = r"<(?P<email>[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)>";
const PERSON_AT_REGEX: &str = r#"(?P<name>\w+(\s\w+)*)\s*at\s*<"(?P<email>[a-zA-Z0-9_.+-]+ at [a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)">"#;
const PERSON_AT_NO_NAME_REGEX: &str = r#""(?P<email>[a-zA-Z0-9_.+-]+ at [a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)""#;

const PATCH_SUBJECT_REGEX: &str = r"^\[PATCH( v(?P<version>\d+))? (?P<index>\d+)/(?P<total>\d+)\] (?P<tags>([^:]+:)*)(?P<description>.+)$";
const TAGGED_SUBJECT_REGEX: &str = r"^(?P<tags>([^:]+:)+)(?P<description>.+)$";
const SIMPLE_SUBJECT_REGEX: &str = r"^(?P<description>.+)$";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Header<'input> {
    From(Person<'input>),
    Date(DateTime<Utc>),
    Author(Person<'input>),
    Subject(Subject<'input>),
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
            "subject" => Ok(Header::Subject(value.try_into()?)),
            _ => Ok(Header::Other(key, value)),
        }
    }
}

impl Display for Header<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Header::From(person) => cwrite!(f, "<s><b>From:</b></s> {}", person),
            Header::Date(date) => cwrite!(f, "<s><g>Date:</g></s> <g>{}</g>", date.to_rfc2822()),
            Header::Author(person) => cwrite!(f, "<s><r>Author:</r></s> {}", person),
            Header::Subject(subject) => cwrite!(f, "<s><y>Subject:</y></s> {}", subject),
            Header::Other(key, value) => cwrite!(f, "<c>{}:</c> {}", key, value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person<'input> {
    pub name: Option<&'input str>,
    pub email: Email<'input>,
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
        let email = captures.name("email").ok_or("Invalid email")?.as_str().try_into()?;

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
        cwrite!(f, "<m>{}@{}</m>", self.user, self.domain)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Subject<'input> {
    Simple(&'input str),
    Tagged {
        tags: Vec<&'input str>,
        description: &'input str,
    },
    Patch {
        version: Option<usize>,
        index: Option<(usize, usize)>,
        tags: Vec<&'input str>,
        description: &'input str,
    }
}

impl<'input> TryFrom<&'input str> for Subject<'input> {
    type Error = String;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let re_patch = Regex::new(PATCH_SUBJECT_REGEX).unwrap();
        let re_tagged = Regex::new(TAGGED_SUBJECT_REGEX).unwrap();
        let re_simple = Regex::new(SIMPLE_SUBJECT_REGEX).unwrap();

        if let Some(captures) = re_patch.captures(value) {
            let version = captures.name("version").map(|v| v.as_str().parse::<usize>().ok()).flatten();
            let base = captures.name("index").map(|t| t.as_str().parse::<usize>().ok()).flatten();
            let total = captures.name("total").map(|t| t.as_str().parse::<usize>().ok()).flatten();
            let tags = captures.name("tags").ok_or("Invalid tags")?.as_str().split(':').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            let description = captures.name("description").map(|d| d.as_str()).map(|s| s.trim()).ok_or("Invalid description")?;

            let index = match (base, total) {
                (Some(base), Some(total)) => Some((base, total)),
                _ => None,
            };
            Ok(Subject::Patch { version, index, tags, description })
        } else if let Some(captures) = re_tagged.captures(value) {
            let tags = captures.name("tags").ok_or("Invalid tags")?.as_str().split(':').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            let description = captures.name("description").map(|d| d.as_str()).map(|s| s.trim()).ok_or("Invalid description")?;
            
            Ok(Subject::Tagged { tags, description })
        } else if let Some(captures) = re_simple.captures(value) {
            let description = captures.name("description").map(|d| d.as_str()).map(|s| s.trim()).ok_or("Invalid description")?;

            Ok(Subject::Simple(description))
        } else {
            Err("Invalid subject".to_string())
        }
    }
}

impl Display for Subject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::Simple(description) => write!(f, "{}", description),
            Subject::Tagged { tags, description } => write!(f, "{}: {}", tags.join(": "), description),
            Subject::Patch { version, index, tags, description } => {
                let version = version.map(|v| format!(" v{}", v)).unwrap_or_default();
                let index = index.map(|(i, t)| cformat!(" <r>{}/{}</r>", i, t)).unwrap_or_default();
                let tags = if tags.len() > 0 { tags.join(": ") + ": " } else { "".to_string() };

                cwrite!(f, "<y>[PATCH{}</y>{}<y>]</y> <g>{}</>{}", version, index, tags, description)
            }
        }
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
    fn format_header() {
        let header = Header::Other("SomeHeader", "SomeValue");
        println!("{}", header);
        assert_eq!(header.to_string(), format!("\u{1b}[36mSomeHeader:\u{1b}[39m SomeValue"));
    }

    #[test]
    fn parse_person() {
        let person = Person::try_from("Foo Bar <foo.bar@bar.com>");
        assert!(person.is_ok());

        let person = person.unwrap();
        assert_eq!(person.name, Some("Foo Bar"));
        assert_eq!(person.email, "foo.bar@bar.com".try_into().unwrap());
    }

    #[test]
    fn parse_subject() {
        let subject = Subject::try_from("[PATCH v1 1/1] foo: bar: baz");
        assert!(subject.is_ok());

        let subject = subject.unwrap();
        assert_eq!(subject, Subject::Patch { version: Some(1), index: Some((1, 1)), tags: vec!["foo", "bar"], description: "baz" });

        let subject = Subject::try_from("[PATCH 0/2] some example patch");
        assert!(subject.is_ok());

        let subject = subject.unwrap();
        assert_eq!(subject, Subject::Patch { version: None, index: Some((0, 2)), tags: vec![], description: "some example patch" });

        let subject = Subject::try_from("foo: bar");
        assert!(subject.is_ok());
        
        let subject = subject.unwrap();
        assert_eq!(subject, Subject::Tagged { tags: vec!["foo"], description: "bar" });
    }

    #[test]
    fn format_subject() {
        let subject = Subject::Patch { version: Some(1), index: Some((1, 1)), tags: vec!["foo", "bar"], description: "baz" };
        assert_eq!(subject.to_string(), "\u{1b}[43m[PATCH v1 1/1]\u{1b}[49m \u{1b}[32mfoo: bar: \u{1b}[39mbaz");
        println!("{}", subject);

        let subject = Subject::Patch { version: None, index: Some((0, 2)), tags: vec![], description: "some example patch" };
        assert_eq!(subject.to_string(), "[PATCH 0/2] some example patch");

        let subject = Subject::Tagged { tags: vec!["foo"], description: "bar" };
        assert_eq!(subject.to_string(), "foo: bar");

        let subject = Subject::Simple("baz foo barbar");
        assert_eq!(subject.to_string(), "baz foo barbar");
    }
}