use std::io::Cursor;

use base64;
use base64::{Engine as _, engine::general_purpose};
use image::{DynamicImage, ImageFormat};

/// Get the base64 string from an image.
///
/// # Arguments
///
/// * `img` - a DynamicImage instance.
///
/// # Returns
///
/// String - a string that holds the base64 encoded image.
pub fn get_base64_from_image(img: &DynamicImage) -> String {
    let mut buffer = Cursor::new(Vec::new());

    // Create a buffer to hold the encoded image
    // Encode the image to PNG format
    // TODO improve error handling
    img.write_to(&mut buffer, ImageFormat::Png).expect("Failed to encode image");
    let image_bytes = buffer.into_inner();
    let base64_string = general_purpose::STANDARD.encode(image_bytes);

    return base64_string;
}

/// Get the base64 string from an image. Uses the old base64::encode function which is deprecated.
///
/// # Arguments
///
/// * `img` - a DynamicImage instance.
///
/// # Returns
///
/// String - a string that holds the base64 encoded image.
#[deprecated]
pub fn get_base64_from_image_old(img: &DynamicImage) -> String {
    // Create a buffer to hold the encoded image
    let mut buffer = Cursor::new(Vec::new());
    // Encode the image to PNG format
    // TODO improve error handling
    img.write_to(&mut buffer, ImageFormat::Png).expect("Failed to encode image");
    let image_bytes = buffer.into_inner();
    let base64_string = base64::encode(image_bytes);

    return base64_string;
}

/// Save a JSON string to a file. Technically this just saves text to a file.
///
/// # Arguments
///
/// * `json_data` - a string slice that holds the JSON data.
/// * `file_path` - a string slice that holds the path to the file.
///
/// # Returns
///
/// Result<(), Box<dyn std::error::Error>> - an empty result if successful, otherwise an error.
pub async fn save_json_to_file(json_data: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(file_path)?;
    let write_result = file.write_all(json_data.as_bytes());
    match write_result {
        Ok(_) => {
            println!("JSON saved to: {}", file_path);
            Ok(())
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(e.into())
        }
    }
}

pub fn get_filename_from_path(file_path: &str) -> &str {
    let file_folder = match file_path.split('/').collect::<Vec<&str>>().last() {
        Some(file) => file,
        None => file_path,
    };
    let filename: &str = file_folder.split('.').collect::<Vec<&str>>()[0];
    filename
}

/// Filter markdown JSON data.
///
/// # Arguments
///
/// * `json_data` - a string slice that holds the JSON data.
///
/// # Returns
///
/// String - a string that holds the filtered JSON data.
pub fn filter_markdown_json(json_data: &str) -> String {
    let mut filtered_json = json_data.replace("\\n", "\n");

    // remove ```json from the start and ``` from the end
    filtered_json = filtered_json.replace("```json", "");
    filtered_json = filtered_json.replace("```", "");

    // remove extra backslashes
    filtered_json = filtered_json.replace("\\\"", "\"");
    filtered_json = filtered_json.replace("\t", "\t");
    filtered_json = filtered_json.replace("\\r", "\r");
    filtered_json = filtered_json.replace("\\\\", "\\");
    filtered_json = filtered_json.replace("\\/", "/");

    // strip leading and trailing whitespace
    filtered_json = filtered_json.trim().to_string();

    filtered_json
}

#[cfg(test)]
mod tests {
    use image::ImageReader as ImageReader;

    use super::*;

    #[test]
    fn test_get_base64_from_image() {
        let img = ImageReader::open("samples/invoice.png").unwrap().decode().unwrap();
        let base64_string = get_base64_from_image(&img);
        let base64_string_old = get_base64_from_image_old(&img);
        assert_eq!(base64_string, base64_string_old);
    }

    #[tokio::test]
    async fn test_save_json_to_file() {
        let json_data = r#"{"name": "John Doe", "age": 30}"#;
        let file_path = "samples/test.json";
        let result = save_json_to_file(json_data, file_path);
        assert_eq!(result.await.is_ok(), true);
    }

    #[test]
    fn test_get_filename_from_path() {
        let file_path = "samples/invoice.png";
        let filename = get_filename_from_path(file_path);
        assert_eq!(filename, "invoice");

        let file_path2 = "invoice.png";
        let filename2 = get_filename_from_path(file_path2);
        assert_eq!(filename2, "invoice");
    }

    #[test]
    fn test_filter_markdown_json() {
        let json_data = r#"```json {"name": "John Doe","age": 30}```"#;
        let filtered_json = filter_markdown_json(json_data);
        assert_eq!(filtered_json, r#"{"name": "John Doe","age": 30}"#);
    }
}