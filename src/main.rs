use std::env;

use dotenvy::dotenv;

use internals::images_from_pdf;

use crate::internals::gpt4o;

mod internals;

#[tokio::main]
async fn main() {
    // load environment variables from .env file
    dotenv().expect(".env file not found");

    let target_document = "Resume.pdf";
    let openai_api_key = env::var("OPENAI_API_KEY").expect("failed to load OpenAI API key");

    // Step 1: Extract images from the PDF file
    let image_paths = images_from_pdf::extract_images_from_pdf(target_document).unwrap();

    // Step 2: Run OCR on the extracted images
    for image_path in image_paths {
        let response_json = internals::gpt4o::run_ocr_on_image(internals::gpt4o::create_openai_client(&openai_api_key).unwrap(), image_path.as_str()).await;
        match response_json {
            Ok(json) => {
                println!("{}", json);
                let json_path = "output/".to_string() + &image_path + ".json";
                let _ = gpt4o::save_json_to_file(json.as_str(), json_path.as_str()).await;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

