use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub text: String,
    pub links: Option<Vec<String>>,
}
