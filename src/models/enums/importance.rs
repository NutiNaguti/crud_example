use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
// #[serde(rename_all = "lowercase")]
pub enum Importance {
    Hight,
    Medium,
    Low,
}
