use std::process::{Command, Stdio}; 
use std::io::{self, BufReader, Read}; 

fn main() -> io::Result<()> {
    let pdf_path = "/Users/spencer/Documents/email_papers_script/sample_pdfs/test.pdf"; 

    let mut child = Command::new("python")
        .arg("/Users/spencer/Documents/pdf_index/pdf_indexer/python/extract_pdf_text.py")
        .arg(pdf_path)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute Python script");

    // read text extracted from py script's stdout 
    let stdout = child.stdout.take().expect("Failed to capture stdout"); 
    let mut reader = BufReader::new(stdout); 
    let mut extracted_text = String::new(); 
    reader.read_to_string(&mut extracted_text)?; 

    // make sure py process finishes 
    child.wait()?; 

    // extract above logic into its own function (and return the stdout/extracted_text)

    // process rest of stuff here 
    println!("Extracted text:\n{}", extracted_text); 

    Ok(())
}