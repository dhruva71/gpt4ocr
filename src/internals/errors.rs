use std::fmt;

#[derive(Debug, Clone)]
pub struct APIKeyError {
    message: String,
}

impl APIKeyError {
    pub(crate) fn new(message: &str) -> Self {
        APIKeyError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for APIKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}