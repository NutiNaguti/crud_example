use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    text: String,
    links: Option<Vec<String>>,
}
