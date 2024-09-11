use std::collections::HashMap; 

type DocumentID = String; 
type Position = usize; 

pub struct InvIndex {
    index: HashMap<String, HashMap<DocumentID, Vec<Position>>> // word -> doc ID -> positions in document
}

impl InvIndex {

}