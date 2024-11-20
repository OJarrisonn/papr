use crate::{config::renderer, mailbox::{self, message::{self, header}}};

pub fn mailbox(config: renderer::Renderer, mailbox: mailbox::Mailbox) -> String {
    let mut rendered = String::new();
    
    for message in mailbox.messages {
        rendered += message(config, message);
    }

    rendered
}

pub fn message(config: renderer::Renderer, message: message::Message) -> String {
    let mut rendered = String::new();

    for h in message.headers {
        let (key, value) = header(config, h);
        rendered += &format!(
            "{}: {}\n",
            key,
            value
        );
    }

    rendered += "\n";

    rendered += match &message.body {
        message::body::Body::Simple(body) => *body,
        message::body::Body::WithFrontMatter { front_matter, footers, body } => {
            let mut rendered = String::new();

            rendered += front_matter;

            for (key, value) in footers {
                rendered += &format!("{}: {}\n", key, value);
            }

            rendered += "\n";
            rendered += body;

            &rendered
        }
        message::body::Body::OnlyFrontMatter { front_matter, footers } => {
            let mut rendered = String::new();

            rendered += front_matter;

            for (key, value) in footers {
                rendered += &format!("{}: {}\n", key, value);
            }

            rendered += "\n";

            &rendered
        }
    };

    rendered
}

pub fn header(config: renderer::Renderer, header: header::Header) -> (String, String) {
    match header {
        header::Header::From(person) => format!("From: {}\n", person(config, person)),
        header::Header::Date(date) => format!("Date: {}\n", date),
        header::Header::Author(person) => format!("Author: {}\n", person(config, person)),
        header::Header::Subject(subject) => format!("Subject: {}\n", subject),
        header::Header::Other(key, value) => format!("{}: {}\n", key, value),
    }
}

pub fn email(config: renderer::Renderer, email: header::Email) -> String {
    let header::Email { user, domain } = email;

    format!(
        "{}{}{}",
        config.email.user_color.paint(user),
        config.email.domain_separator,
        config.email.domain_color.paint(domain)
    )
}

pub fn person(config: renderer::Renderer, person: header::Person) -> String {
    let header::Person { name, email: e } = person;

    match name {
        Some(name) => format!("{} {}", name, email(config, e)),
        None => email(
            renderer::Renderer {
                email: renderer::Email {
                    ommit_domain: config.person.ommit_email,
                    ..config.email
                },
                ..config
            },
            e,
        ),
    }
}
