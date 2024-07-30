use kalosm::vision::*;

#[tokio::main]
async fn extract_text_from_image() {
    let mut model = Ocr::builder().build().await.unwrap();
    let image = image::open("examples/ocr.png").unwrap();
    let text = model
        .recognize_text(OcrInferenceSettings::new(image).unwrap())
        .unwrap();

    println!("{}", text);
}