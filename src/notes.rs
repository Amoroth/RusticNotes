use std::io::Write;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RusticNote {
    pub content: String,
}

impl RusticNote {
    pub fn new(content: String) -> Self {
        RusticNote { content }
    }
}

// todo move to a separate file
pub fn save_note(note: &RusticNote) {
    println!("Saving note: {}", note.content);

    let note_json = serde_json::to_string(note).unwrap();
    println!("Serialized note: {}", note_json);

    // todo save incrementally, don't overwrite whole file on every save
    let mut saved_notes: Vec<RusticNote> = load_all_notes();

    saved_notes.push(note.clone());

    let serialized_notes = serde_json::to_string(&saved_notes).unwrap();
    match std::fs::File::create("notes.json") {
        Ok(mut file) => {
            if let Err(e) = file.write_all(serialized_notes.as_bytes()) {
                eprintln!("Error writing to file: {}", e);
            } else {
                println!("Note saved successfully.");
            }
        }
        Err(e) => {
            eprintln!("Error creating file: {}", e);
        }
    }
}

pub fn load_all_notes() -> Vec<RusticNote> {
    match std::fs::read_to_string("notes.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}
