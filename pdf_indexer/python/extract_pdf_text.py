import sys 
import pymupdf as pym

def extract_pdf_text(pdf_path): 
    doc = pym.open(pdf_path)
    text = ""

    for page_num in range(len(doc)):
        page = doc[page_num]
        text += page.get_text() 

    print(text)

if __name__ == "__main__":
    if len(sys.argv) != 2: 
        print("Usage: python extract_pdf_text.py <path_to_pdf_here>")
        sys.exit(1)
    
    pdf_path = sys.argv[1]
    extract_pdf_text(pdf_path)