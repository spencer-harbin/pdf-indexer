use std::fs::File;
use std::io::BufWriter;
use pdf_extract::extract_text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdf_path = "example/path/for/now.pdf";

    let text = extract_text(pdf_path)?;

    println!("Extracted text:\n{}", text);

    // save extracted text to txt file
    let output_path = "output.txt";
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    std::io::Write::write_all(&mut writer, text.as_bytes())?;

    Ok(())
}