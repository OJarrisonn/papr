use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Renderer {
    pub frontmatter: Frontmatter,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Frontmatter {
    pub headers: HashMap<String, String>,
}
