use serde::{Deserialize, Serialize};

#[repr(i16)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Ready = 1,
    NotReady = 2,
    Canceled = 3,
}
