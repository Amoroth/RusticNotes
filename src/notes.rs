use std::{io::Write, path::Path};
use serde::{Serialize, Deserialize};
use crate::{print_utils, config};

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
    let config = config::get_config();
    let notes_directory = Path::new(&config.notes_directory);

    match notes_directory.try_exists() {
        Ok(true) => {},
        Ok(false) => {
            if let Err(e) = std::fs::create_dir_all(notes_directory) {
                eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Error creating notes directory: {e}").as_str()));
                return;
            }
        },
        Err(e) => {
            eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Error checking notes directory: {e}").as_str()));
            return;
        },
    }

    // todo save incrementally, don't overwrite whole file on every save
    // idea: save offsets for every note and update just the changed note
    let serialized_notes = serde_json::to_string(&notes).unwrap();
    match std::fs::File::create(notes_directory.join("notes.json")) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(serialized_notes.as_bytes()) {
                eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Error writing to file: {e}").as_str()));
            } else {
                println!("{}", print_utils::colorize(print_utils::Color::success(), "Note saved successfully."));
            }
        }
        Err(e) => {
            eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Error creating file: {e}").as_str()));
        }
    }
}

pub fn save_note(note: &RusticNote) {
    println!("Saving note: {}", note.content);

    let note_json = serde_json::to_string(note).unwrap();
    println!("Serialized note: {note_json}");

    let mut saved_notes: Vec<RusticNote> = load_all_notes();

    let exiting_note = saved_notes.clone().into_iter().enumerate().find(|(_, n)| n.id == note.id);
    saved_notes.push(note.clone());

    if exiting_note.is_some() {
        let last_index = saved_notes.len() - 1;
        saved_notes.swap(exiting_note.unwrap().0, last_index);
        saved_notes.pop();
    }

    save_notes(saved_notes);
}

pub fn load_all_notes() -> Vec<RusticNote> {
    let config = config::get_config();
    let notes_directory = Path::new(&config.notes_directory);
    
    if !notes_directory.exists() {
        return vec![];
    }

    match std::fs::read_to_string(notes_directory.join("notes.json")) {
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
pub fn slow_search(notes: &[RusticNote], query: &str) -> Vec<RusticNote> {
    notes.iter()
        .filter(|&note| note.content.contains(query))
        .cloned()
        .collect()
}
