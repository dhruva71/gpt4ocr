mod internals;

use internals::images_from_pdf;

fn main()  {
    let file_path = "Resume.pdf";

    let _ = internals::gpt4o::create_openai_client("");

    let result = images_from_pdf::extract_images_from_pdf(file_path);
    match result {
        Ok(v) => {
            println!("Images extracted successfully: {:?}", v);
        },
        Err(e) => println!("Error: {}", e),
    }
}

