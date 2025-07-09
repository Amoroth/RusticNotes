use std::io::Write;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RusticNote {
    pub id: u32,
    pub content: String,
    pub tags: Vec<String>
}

impl RusticNote {
    pub fn new(content: String, tags: Vec<String>) -> Self {
        RusticNote { id: get_next_id(), content, tags }
    }
}

pub fn save_notes(notes: Vec<RusticNote>) {
    // todo save incrementally, don't overwrite whole file on every save
    // idea: save offsets for every note and update just the changed note
    let serialized_notes = serde_json::to_string(&notes).unwrap();
    match std::fs::File::create("notes.json") {
        Ok(mut file) => {
            if let Err(e) = file.write_all(serialized_notes.as_bytes()) {
                eprintln!("Error writing to file: {e}");
            } else {
                println!("Note saved successfully.");
            }
        }
        Err(e) => {
            eprintln!("Error creating file: {e}");
        }
    }
}

// todo move to a separate file
pub fn save_note(note: &RusticNote) {
    println!("Saving note: {}", note.content);

    let note_json = serde_json::to_string(note).unwrap();
    println!("Serialized note: {note_json}");

    let mut saved_notes: Vec<RusticNote> = load_all_notes();
    saved_notes.push(note.clone());
    save_notes(saved_notes);
}

pub fn load_all_notes() -> Vec<RusticNote> {
    match std::fs::read_to_string("notes.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}

pub fn get_note_by_id(id: u32) -> Option<RusticNote> {
    let notes = load_all_notes();
    notes.into_iter().find(|note| note.id == id)
}

pub fn remove_note_by_id(id: u32) {
    let mut updated_notes = load_all_notes();
    updated_notes.retain(|note| note.id != id);
    save_notes(updated_notes);
}

pub fn get_next_id() -> u32 {
    let notes = load_all_notes();
    let mut biggest_id = 0;
    for note in notes {
        if note.id > biggest_id {
            biggest_id = note.id;
        }
    }
    biggest_id + 1
}

// realistically, i should use something like ripgrep here, read up on Boyer–Moore string search algo and maybe implement it?
pub fn slow_search(query: &str) -> Vec<RusticNote> {
    let notes = load_all_notes();
    notes.into_iter()
        .filter(|note| note.content.contains(query))
        .collect()
}