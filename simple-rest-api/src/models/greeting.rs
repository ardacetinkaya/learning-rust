use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Greeting {
    pub message: String,
}
