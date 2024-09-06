mod extract_pdf_text;

use std::io; 

fn main() -> io::Result<()> {
    let pdf_path = "/Users/spencer/Documents/email_papers_script/sample_pdfs/test.pdf"; 
    let python_script_path = "/Users/spencer/Documents/pdf_index/pdf_indexer/python/extract_pdf_text.py"; 

    let extracted_text = extract_pdf_text::extract_pdf_text(pdf_path, python_script_path)?; 

    // process rest of stuff here 
    println!("Extracted text:\n{}", extracted_text); 

    Ok(())
}