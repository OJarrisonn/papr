use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::Color;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Renderer {
    pub frontmatter: Frontmatter,
    pub email: Email,
    pub person: Person,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Frontmatter {
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub user_color: Color,
    pub domain_separator: String,
    pub domain_color: Color,
    pub ommit_domain: bool,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Person {
    pub name_color: Color,
    pub ommit_email: bool,
}


impl Default for Email {
    fn default() -> Self {
        Email {
            user_color: Color::default(),
            domain_separator: "@".into(),
            domain_color: Color::default(),
            ommit_domain: false,
            prefix: "<".to_string().into(),
            suffix: ">".to_string().into(),
        }
    }
}