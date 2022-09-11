use serde::{Deserialize, Serialize};

#[repr(i16)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Importance {
    Hight = 1,
    Medium = 2,
    Low = 3,
}
