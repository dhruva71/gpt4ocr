use std::env;
use openai_api_rs::v1::api::OpenAIClient;
use crate::internals::errors;

pub fn create_openai_client(api_key: &str) -> Result<OpenAIClient, errors::APIKeyError> {
    let openai_api_key: String;
    if !api_key.is_empty() {
        openai_api_key = api_key.to_string();
        dbg!("OpenAI API key is empty.");
    } else {
        match env::var("OPENAI_API_KEY") {
            Ok(key) => {
                openai_api_key = key;
                dbg!("OpenAI API key loaded successfully.", &openai_api_key);
            }
            Err(e) => {
                dbg!("Error: failed to load OpenAI API key.\nDetails: {}", e);
                return Err(errors::APIKeyError::new("Failed to load OpenAI API key."));
            }
        }
    }
    dbg!("Creating OpenAI client with API key:", &openai_api_key);
    let client = OpenAIClient::new(openai_api_key);
    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_openai_client() {
        let api_key = "";
        let client = create_openai_client(api_key);
        assert!(client.is_ok());
    }
}