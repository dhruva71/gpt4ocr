mod internals;

use internals::images_from_pdf;
use dotenvy::dotenv;
use std::env;
use crate::internals::gpt4o;

#[tokio::main]
async fn main()  {
    // load environment variables from .env file
    dotenv().expect(".env file not found");

    let openai_api_key = env::var("OPENAI_API_KEY").expect("failed to load OpenAI API key");

    let response_json = internals::gpt4o::run_ocr_on_image(internals::gpt4o::create_openai_client(&openai_api_key).unwrap(), "output/Resume2-0-1.png").await;
    match response_json {
        Ok(json) => {
            println!("{}", json);
            gpt4o::save_json_to_file(json.as_str(), "output/Resume2-0-1.json").await;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

