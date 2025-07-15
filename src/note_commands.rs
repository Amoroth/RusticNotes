use crate::cli_command::{CliCommandBuilder, CliCommand, CliCommandOption};
use crate::notes;
use std::{collections::HashMap, io::Write};

pub fn build_new_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("new")
        .add_alias("add")
        .set_description("Create a new note")
        .set_optional(false)
        .add_argument("note")
        .add_option(
            &CliCommandOption {
                name: "tag".to_string(),
                short_name: Some("t".to_string()),
                description: Some("Add a tag to the note".to_string()),
                is_flag: false
            }
        ).set_action(|args: HashMap<String, Vec<String>>| {
            if let Some(note) = args.get("note") {
                let note_content = note.last().unwrap_or(&String::from("")).to_string();
                println!("Creating new note: {note_content}");
                let tags: Vec<String> = args.get("tag").unwrap_or(&vec![]).clone();
                if !tags.is_empty() {
                    println!("With tags: {tags:?}");
                }
                let new_note = notes::RusticNote::new(note_content.clone(), tags);
                notes::save_note(&new_note);
            } else {
                eprintln!("Error: Note name is required.");
            }
        }).build()
}

pub fn build_list_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("list")
        .add_alias("ls")
        .set_description("List all notes")
        .set_optional(false)
        .add_option(
            &CliCommandOption {
                name: "tag".to_string(),
                short_name: Some("t".to_string()),
                description: Some("Search by a tag".to_string()),
                is_flag: false
            }
        ).set_action(|args: HashMap<String, Vec<String>>| {
            let mut notes = notes::load_all_notes();
            if notes.is_empty() {
                println!("No notes found.");
            } else {
                let tags = args.get("tag").unwrap_or(&vec![]).clone();

                if !tags.is_empty() {
                    notes.retain(|note| note.tags.iter().any(|tag| tags.contains(tag)));
                }

                println!("Notes:");
                for note in notes {
                    println!("{}. {}", note.id, note.content);
                }
            }
        }).build()
}

pub fn build_get_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("get")
        .set_description("Get a single note by its id")
        .set_optional(false)
        .add_argument("id")
        .set_action(|args: HashMap<String, Vec<String>>| {
            if let Some(id_str) = args.get("id").and_then(|v| v.last()) {
                if let Ok(id) = id_str.parse::<u32>() {
                    if let Some(note) = notes::get_note_by_id(id) {
                        println!("{}", note.content);
                    } else {
                        eprintln!("Note with id {id} not found.");
                    }
                } else {
                    eprintln!("Invalid id: {id_str}");
                }
            } else {
                eprintln!("Error: Note id is required.");
            }
        }).build()
}

pub fn build_delete_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("delete")
        .add_alias("remove")
        .add_alias("rm")
        .set_description("Delete a single note by its id")
        .set_optional(false)
        .add_argument("id")
        .set_action(|args: HashMap<String, Vec<String>>| {
            if let Some(id_str) = args.get("id").and_then(|v| v.last()) {
                if let Ok(id) = id_str.parse::<u32>() {
                    if let Some(_) = notes::get_note_by_id(id) {
                        notes::remove_note_by_id(id);
                    } else {
                        eprintln!("Note with id {id} not found.");
                    }
                } else {
                    eprintln!("Invalid id: {id_str}");
                }
            } else {
                eprintln!("Error: Note id is required.");
            }
        }).build()
}

pub fn build_search_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("search")
        .set_description("Search for a note by a query string")
        .set_optional(false)
        .add_argument("query")
        .set_action(|args: HashMap<String, Vec<String>>| {
            if let Some(query) = args.get("query").and_then(|v| v.last()) {
                let notes = notes::slow_search(query);
                if notes.is_empty() {
                    println!("No notes found.");
                } else {
                    println!("Notes:");
                    for note in notes {
                        println!("{}. {}", note.id, note.content);
                    }
                }
            } else {
                eprintln!("Error: Query is required.");
            }
        }).build()
}

pub fn build_edit_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("edit")
        .set_description("Edit a single note by its id")
        .set_optional(false)
        .add_argument("id")
        .add_option(
            &CliCommandOption {
                name: "message".to_string(),
                short_name: Some("m".to_string()),
                description: Some("Replace note by this string. If --interactive option is passed, it is discarded.".to_string()),
                is_flag: false
            }
        ).add_option(
            &CliCommandOption {
                name: "interactive".to_string(),
                short_name: Some("i".to_string()),
                description: Some("Edit note interactivly through an external editor. One has to be provided through config or it will fail.".to_string()),
                is_flag: false
            }
        ).set_action(|args: HashMap<String, Vec<String>>| {
            let id_str = args.get("id").and_then(|v| v.last());
            let id = match id_str {
                Some(id) => match id.parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        eprintln!("Invalid id: {id}");
                        return;
                    }
                },
                None => {
                    eprintln!("Error: Note id is required.");
                    return;
                }
            };

            let mut note = match notes::get_note_by_id(id) {
                Some(note) => note,
                None => {
                    eprintln!("Note with id {id} not found.");
                    return;
                }
            };

            let edited_note_content = if args.contains_key("interactive") || !args.contains_key("message") {
                let config = notes::get_config();
                let editor = match config.editor {
                    Some(e) => e,
                    None => {
                        eprintln!("No editor available.");
                        return;
                    }
                };
                
                // save note to temporary file
                let temp_file_path = format!("/tmp/rustic_note_{}.txt", id);
                let mut file = match std::fs::File::create(&temp_file_path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Error creating temporary file: {e}");
                        return;
                    }
                };
                if let Err(e) = file.write_all(note.content.trim().as_bytes()) {
                    eprintln!("Error writing note to temporary file: {e}");
                    return;
                }
    
                std::process::Command::new(editor)
                    .arg(&temp_file_path)
                    .spawn()
                    .expect("Error: Failed to run editor")
                    .wait()
                    .expect("Error: Editor returned a non-zero status");
                
                // read the edited note back
                match std::fs::read_to_string(&temp_file_path) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("Error reading edited note: {e}");
                        return;
                    }
                }
            } else {
                args.get("message").and_then(|v| v.last()).unwrap_or(&String::new()).to_string()
            };

            note.content = edited_note_content.trim().to_string();
            notes::save_note(&note);
        }).build()
}