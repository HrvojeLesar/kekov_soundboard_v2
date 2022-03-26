use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Serialize, Deserialize)]
pub struct GenericSuccess {
    pub success: String,
}

impl GenericSuccess {
    pub fn new(message: &str) -> Self {
        return GenericSuccess {
            success: message.to_owned(),
        };
    }
}

impl Default for GenericSuccess {
    fn default() -> Self {
        return Self {
            success: "success".to_owned(),
        };
    }
}
