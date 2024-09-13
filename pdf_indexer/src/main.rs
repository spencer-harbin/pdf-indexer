mod extract_pdf_text;
mod search_text;
mod index_text;

use std::io;

use index_text::InvIndex; 

fn main() -> io::Result<()> {
    let mut index = InvIndex::new(); 

    let pdf_path = "/Users/spencer/Documents/email_papers_script/sample_pdfs/test.pdf"; 
    let python_script_path = "/Users/spencer/Documents/pdf_index/pdf_indexer/python/extract_pdf_text.py"; 

    let extracted_text = extract_pdf_text::extract_pdf_text(pdf_path, python_script_path)?; 

    // process rest of stuff here 
    println!("Extracted text:\n{}", extracted_text); 

    // index the test doc
    index.index_document("doc1.txt".to_string(), &extracted_text); 

    index.print_index();

    Ok(())
}