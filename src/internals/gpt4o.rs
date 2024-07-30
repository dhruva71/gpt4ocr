use std::env;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::ChatCompletionRequest;
use crate::internals::errors;

enum Model {
    GPT4o,
    Gpt4oMini,
}

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

pub fn run_ocr_on_image(image_path: &str, model: Model) {
    match model {
        Model::GPT4o => {
            dbg!("Running OCR on image using GPT4o model.");
        },
        Model::Gpt4oMini => {
            dbg!("Running OCR on image using Gpt4oMini model.");
        }
    }

    // run the OpenAI request
    let request = ChatCompletionRequest::new(
        model.to_string(),

    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_openai_client() {
        let api_key = "";
        let client = create_openai_client(api_key);
        assert!(!client.is_ok()); // should return an error since the API key is empty
    }
}