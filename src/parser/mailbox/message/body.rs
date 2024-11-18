use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body<'input> {
    Simple(&'input str),
    WithFrontMatter {
        front_matter: &'input str,
        footers: Vec<(&'input str, &'input str)>,
        body: &'input str,
    },
    OnlyFrontMatter {
        front_matter: &'input str,
        footers: Vec<(&'input str, &'input str)>,
    },
}

impl Body<'_> {
    pub fn front_matter_only(self) -> Self {
        match self {
            Body::WithFrontMatter {
                front_matter,
                footers,
                ..
            } => Body::OnlyFrontMatter {
                front_matter,
                footers,
            },
            other => other,
        }
    }
}

impl<'input> From<&'input str> for Body<'input> {
    fn from(value: &'input str) -> Self {
        if value.contains("\n---\n") {
            let (front_matter, body) = value.split_once("\n---\n").unwrap();
            let mut footers = Vec::new();
            let mut consume = 0;

            for line in front_matter.lines().rev() {
                if line.trim().is_empty() {
                    break;
                }

                let mut parts = line.splitn(2, ": ");

                if parts.clone().count() == 1 {
                    break;
                }

                let key = parts.next().unwrap().trim();
                let value = parts.next().unwrap_or("").trim();

                footers.push((key, value));
                consume += line.len() + 1;
            }

            Body::WithFrontMatter {
                front_matter: &front_matter[..front_matter.len() - consume].trim(),
                footers,
                body: body.trim(),
            }
        } else {
            Body::Simple(value)
        }
    }
}

impl Display for Body<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Body::Simple(body) => write!(f, "{}", body),
            Body::WithFrontMatter {
                front_matter,
                footers,
                body,
            } => {
                write!(f, "{}\n---\n", front_matter)?;

                for (key, value) in footers {
                    write!(f, "{}: {}\n", key, value)?;
                }

                write!(f, "---\n{}", body)
            }
            Body::OnlyFrontMatter {
                front_matter,
                footers,
            } => {
                write!(f, "{}\n---\n", front_matter)?;

                for (key, value) in footers {
                    write!(f, "{}: {}\n", key, value)?;
                }

                Ok(())
            }
        }
    }
}
