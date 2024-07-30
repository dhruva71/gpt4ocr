use std::env;

use base64::Engine as _;
use image::GenericImageView;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O_MINI;

use crate::internals::{errors, image_handlers, prompts};

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
    dbg!("Creating OpenAI client with API key: {}", &openai_api_key);
    let client = OpenAIClient::new(openai_api_key);
    Ok(client)
}

/// Run OCR on an image using the OpenAI API, and save the extracted text to a JSON file.
pub async fn run_ocr_on_image(client: OpenAIClient, image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // idea from: https://community.openai.com/t/how-to-load-a-local-image-to-gpt4-vision-using-api/533090/3
    // load image as base64
    let image = image::open(image_path).unwrap();
    let image_base64 = image_handlers::get_base64_from_image(image);
    let url_data = String::from(
        // "https://upload.wikimedia.org/wikipedia/commons/5/50/Bitcoin.png",
        "data:image/png;base64,".to_owned() + &image_base64,
    );


    // run the OpenAI request
    let req = ChatCompletionRequest::new(
        GPT4_O_MINI.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::ImageUrl(vec![
                chat_completion::ImageUrl {
                    r#type: chat_completion::ContentType::text,
                    text: Some(String::from(prompts::generate_personal_data_prompt())),
                    image_url: None,
                },
                chat_completion::ImageUrl {
                    r#type: chat_completion::ContentType::image_url,
                    text: None,
                    image_url: Some(chat_completion::ImageUrlType {
                        url: url_data,
                    }),
                },
            ]),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    );

    // log request time
    let start = std::time::Instant::now();
    let result = client.chat_completion(req).await;
    let duration = start.elapsed();
    println!("Request took: {:?}", duration);

    match result {
        Ok(v) => {
            // println!("Images extracted successfully: {:?}", v);
            match &v.choices[0].message.content {
                Some(content) => {
                    return Ok(content.to_string());
                }
                None => {
                    return Err("No content found.".into());
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok("Unknown error.".to_string())
}

pub async fn save_json_to_file(json_data: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
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