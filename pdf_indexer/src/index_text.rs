use std::collections::HashMap; 

type DocumentID = String; 
type Position = usize; 

pub struct InvIndex {
    index: HashMap<String, HashMap<DocumentID, Vec<Position>>> // word -> doc ID -> positions in document
}

impl InvIndex {
    pub fn new() -> Self {
        InvIndex{
            index: HashMap::new() 
        }
    }

    pub fn index_document(&mut self, doc_id: DocumentID, text: &str) {
        for (position, word) in text.split_whitespace().enumerate() {
            let word = word.to_lowercase(); 

            let entry = self.index.entry(word).or_insert_with(HashMap::new); 
            let doc_entry = entry.entry(doc_id.clone()).or_insert_with(Vec::new); 
            doc_entry.push(position); 

        }
    }

    // delete later
    pub fn print_index(&self) {
        for (word, docs) in &self.index {
            println!("Word: '{}", word); 
            for (doc_id, positions) in docs {
                println!("Document: {}, Positions: {:?}", doc_id, positions)
            }
        }
    }
}