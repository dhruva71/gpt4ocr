use std::io::Cursor;

use base64;
use base64::Engine;
use image::{DynamicImage, ImageFormat};

pub fn get_base64_from_image(img: DynamicImage) -> String {
    // Create a buffer to hold the encoded image
    let mut buffer = Cursor::new(Vec::new());
    // Encode the image to PNG format
    // TODO improve error handling
    img.write_to(&mut buffer, ImageFormat::Png).expect("Failed to encode image");
    let image_bytes = buffer.into_inner();
    let base64_string = base64::encode(image_bytes);

    return base64_string;
}
