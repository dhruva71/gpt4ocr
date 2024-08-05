use std::env;

use dotenvy::dotenv;

use gpt4ocr::gpt4o;

#[tokio::main]
async fn main() {
    // load environment variables from .env file
    dotenv().expect(".env file not found");

    let target_document = "pdfs/Resume.pdf";
    let openai_api_key = env::var("OPENAI_API_KEY").expect("failed to load OpenAI API key");

    let generated_jsons = gpt4o::extract_json_from_pdf(target_document, &openai_api_key, "", false).await;
    match generated_jsons {
        Ok(jsons) => {
            println!("JSONs generated: {:?}", jsons);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

