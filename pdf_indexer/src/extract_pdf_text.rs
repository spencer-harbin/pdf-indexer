use std::process::{Command, Stdio}; 
use std::io::{self, BufReader, Read}; 

pub fn extract_pdf_text(pdf_path: &str, python_script_path: &str) -> io::Result<String> {
    let mut child = Command::new("python")
    .arg(python_script_path)
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

    Ok(extracted_text)
}