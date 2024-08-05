use pdf2image::{PDF, PDF2ImageError, RenderOptionsBuilder};
use crate::file_handlers;

/// Extract images from a PDF file and save them to the output directory.
///
/// # Arguments
/// file_path - a string slice that holds the path to the PDF file.
///
/// # Returns
/// Result<Vec<String>, PDF2ImageError> - a vector of strings that holds the paths to the saved images.
pub fn extract_images_from_pdf(file_path: &str) -> Result<Vec<String>, PDF2ImageError> {

    println!("Extracting images from: {}", file_path);

    let pdf = PDF::from_file(file_path).unwrap();
    let num_pages = pdf.page_count();
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=num_pages),
        RenderOptionsBuilder::default().build()?,
    );

    // split filename at '.' and get the first part
    let filename = file_handlers::get_filename_from_path(file_path);

    let mut output_files: Vec<String> = Vec::new();

    for (i, page) in pages.iter().enumerate() {
        let mut page_count = 0;
        page.iter().for_each(|image| {
            page_count += 1;
            let output_file = format!("output/{}-{}-{}.png", filename, i, page_count);
            output_files.push(output_file.clone());
            image.save(output_file).unwrap();
        });
    }

    return Ok(output_files);
}

