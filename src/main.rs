mod internals;

use internals::images_from_pdf;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main()  {
    // load environment variables from .env file
    dotenv().expect(".env file not found");

    let openai_api_key = env::var("OPENAI_API_KEY").expect("failed to load OpenAI API key");

    let _ = internals::gpt4o::run_ocr_on_image(internals::gpt4o::create_openai_client(&openai_api_key).unwrap(), "output/Resume2-0-1.png").await;
}

