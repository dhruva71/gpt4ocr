use std::env;
use std::error::Error;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O_MINI;

use crate::{errors, file_handlers, images_from_pdf};

pub fn create_openai_client(api_key: &str) -> Result<OpenAIClient, errors::APIKeyError> {
    let openai_api_key: String;
    if !api_key.is_empty() {
        openai_api_key = api_key.to_string();
    } else {
        match env::var("OPENAI_API_KEY") {
            Ok(key) => {
                openai_api_key = key;
            }
            Err(e) => {
                dbg!(e);
                return Err(errors::APIKeyError::new("Failed to load OpenAI API key."));
            }
        }
    }
    dbg!(&openai_api_key);
    let client = OpenAIClient::new(openai_api_key);
    Ok(client)
}

/// Run OCR on an image using the OpenAI API, and a given prompt
///
/// # Arguments
///
/// * `client` - an OpenAIClient instance.
/// * `image_path` - a string slice that holds the path to the image file.
/// * `prompt` - a string slice that holds the prompt for the GPT-4 model.
///
/// # Returns
///
/// Result<String, Box<dyn std::error::Error>> - a string that holds the extracted text from the image.
pub async fn run_ocr_on_image(client: OpenAIClient, image_path: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // idea from: https://community.openai.com/t/how-to-load-a-local-image-to-gpt4-vision-using-api/533090/3
    // load image as base64
    let image = image::open(image_path).unwrap();
    let image_base64 = file_handlers::get_base64_from_image(&image);
    let url_data = String::from(
        "data:image/png;base64,".to_owned() + &image_base64,
    );

    let gpt4o_prompt: String;
    if prompt.is_empty() {
        gpt4o_prompt = "Extract text from the images.".to_string();
    } else {
        gpt4o_prompt = prompt.to_string();
    }


    // run the OpenAI request
    let req = ChatCompletionRequest::new(
        GPT4_O_MINI.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::ImageUrl(vec![
                chat_completion::ImageUrl {
                    r#type: chat_completion::ContentType::text,
                    text: Some(String::from(gpt4o_prompt)),
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

/// Automatically extract text from a PDF file, and generate JSON from it. It then saves the JSON to a file in the same directory as the PDF file.
///
/// # Arguments
///
/// * `target_document` - a string slice that holds the path to the PDF file.
/// * `openai_api_key` - a string slice that holds the OpenAI API key.
/// * `extraction_prompt` - a string slice that holds the prompt for the AI model.
/// * `save_json` - a boolean that determines whether to save the JSON to a file.
///
/// # Returns
///
/// Result<Vec<String>, Box<dyn Err>> - a vector of strings that holds the generated JSONs, or an error.
pub async fn extract_json_from_pdf(target_document: &str, openai_api_key: &String, extraction_prompt: &str, save_json: bool) -> Result<Vec<String>, Box<dyn Error>> {
    // Step 1: Extract images from the PDF file
    let image_paths = images_from_pdf::extract_images_from_pdf(target_document).unwrap();

    // Step 2: Run OCR on the extracted images
    let prompt: String;
    if extraction_prompt.is_empty() {
        println!("No prompt provided. Using default prompt.");
        prompt = "Extract text from the images, and generate JSON from it. Respond only with the json.".to_string();
    } else {
        prompt = extraction_prompt.to_string();
    }

    let mut generated_jsons: Vec<String> = Vec::new();

    for image_path in image_paths {
        let response_json = run_ocr_on_image(create_openai_client(&openai_api_key).unwrap(), image_path.as_str(), prompt.as_str()).await;
        match response_json {
            Ok(json) => {
                let filtered_json = file_handlers::filter_markdown_json(json.as_str());

                // TODO do we need to clone the JSON?
                generated_jsons.push(filtered_json.clone());

                // if save_json is false, skip saving the JSON to a file
                if !save_json {
                    continue;
                }

                println!("{}", json);
                let json_path = image_path + ".json";
                dbg!(&json_path);

                let json_save = file_handlers::save_json_to_file(json.as_str(), json_path.as_str()).await;
                match json_save {
                    Ok(_) => {
                        println!("JSON saved to: {}", json_path);
                    }
                    Err(e) => {
                        dbg!(&e);
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                dbg!(&e);
                return Err(e);
            }
        }
    }

    // return the generated JSONs
    Ok(generated_jsons)
}