use std::io::Cursor;

use base64;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

use image::{DynamicImage, ImageFormat};

pub fn get_base64_from_image(img: DynamicImage) -> String {
    let mut buffer = Cursor::new(Vec::new());

    let b64 = general_purpose::STANDARD.encode(b"hello world~");
    println!("{}", b64);

    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    // Create a buffer to hold the encoded image
    // Encode the image to PNG format
    // TODO improve error handling
    img.write_to(&mut buffer, ImageFormat::Png).expect("Failed to encode image");
    let image_bytes = buffer.into_inner();
    let base64_string = CUSTOM_ENGINE.encode(image_bytes);

    return base64_string;
}

pub async fn save_json_to_file(json_data: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}