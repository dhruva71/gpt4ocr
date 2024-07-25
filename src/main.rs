use pdf2image::{PDF2ImageError, RenderOptionsBuilder, PDF};

fn main()  {
    let file_path = "Resume.pdf";

    let result = extract_images_from_pdf(file_path);
    match result {
        Ok(v) => {
            println!("Images extracted successfully: {:?}", v);
        },
        Err(e) => println!("Error: {}", e),
    }
}

fn extract_images_from_pdf(file_path: &str) -> Result<Vec<String>, PDF2ImageError> {
    // add prefix to the file path
    let mut prefix: String = "pdfs/".to_owned();
    prefix.push_str(file_path);
    println!("Extracting images from: {}", prefix);

    let pdf = PDF::from_file(prefix.as_str()).unwrap();
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=3),
        RenderOptionsBuilder::default().build()?,
    );

    // split filename at '.' and get the first part
    let filename = file_path.split('.').collect::<Vec<&str>>()[0];

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