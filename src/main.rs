use std::env;

use dotenvy::dotenv;

use gpt4ocr::{file_handlers, images_from_pdf};

use gpt4ocr::gpt4o;


#[tokio::main]
async fn main() {
    // load environment variables from .env file
    dotenv().expect(".env file not found");

    let target_document = "pdfs/Resume.pdf";
    let openai_api_key = env::var("OPENAI_API_KEY").expect("failed to load OpenAI API key");

    // Step 1: Extract images from the PDF file
    let image_paths = images_from_pdf::extract_images_from_pdf(target_document).unwrap();

    // Step 2: Run OCR on the extracted images
    let prompt: &str = "Extract text from the images, and generate JSON from it. Respond only with the json.";
    for image_path in image_paths {
        let response_json = gpt4o::run_ocr_on_image(gpt4o::create_openai_client(&openai_api_key).unwrap(), image_path.as_str(), prompt).await;
        match response_json {
            Ok(json) => {
                println!("{}", json);
                let json_path = image_path + ".json";
                dbg!(&json_path);
                let json_save = file_handlers::save_json_to_file(json.as_str(), json_path.as_str()).await;
                match json_save {
                    Ok(_) => {
                        println!("JSON saved to: {}", json_path);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

