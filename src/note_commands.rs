use crate::cli_command::{CliCommandBuilder, CliCommand, CliCommandOption};
use crate::{notes, print_utils};
use std::{collections::HashMap, io::Write};

// todo if returned note is empty, dont save
pub fn build_new_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("new")
        .add_alias("add")
        .set_description("Create a new note")
        .set_optional(false)
        .add_argument("note")
        .add_option(
            &CliCommandOption {
                name: "interactive".to_string(),
                short_name: Some("i".to_string()),
                description: Some("Create note interactivly through an external editor. One has to be provided through config or it will fail.".to_string()),
                is_flag: false
            }
        )
        .add_option(
            &CliCommandOption {
                name: "tag".to_string(),
                short_name: Some("t".to_string()),
                description: Some("Add a tag to the note".to_string()),
                is_flag: false
            }
        ).set_action(|args: HashMap<String, Vec<String>>| {
            let note_content = if args.contains_key("interactive") || !args.contains_key("note") {
                match get_from_editor(None) {
                    Ok(content) => content,
                    Err(EditorOutputError) => {
                        if args.contains_key("note") {
                            args.get("note").and_then(|v| v.last()).unwrap_or(&String::new()).to_string()
                        } else {
                            return;
                        }
                    }
                }
            } else {
                if let Some(note) = args.get("note") {
                    note.last().unwrap_or(&String::from("")).to_string()
                } else {
                    // todo make it easier to write
                    eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Note name is required."));
                    return;
                }
            };

            println!("Creating new note: {note_content}");
            let tags: Vec<String> = args.get("tag").unwrap_or(&vec![]).clone();
            if !tags.is_empty() {
                println!("With tags: {tags:?}");
            }
            let new_note = notes::RusticNote::new(note_content.trim().to_string(), tags);
            notes::save_note(&new_note);
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
                println!("{}", print_utils::colorize(print_utils::Color::warning(), "No notes found."));
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
                        eprintln!("{}", print_utils::colorize(print_utils::Color::warning(), format!("Note with id {id} not found.").as_str()));
                    }
                } else {
                    eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Invalid id: {id_str}").as_str()));
                }
            } else {
                eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Note id is required."));
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
                        eprintln!("{}", print_utils::colorize(print_utils::Color::warning(), format!("Note with id {id} not found.").as_str()));
                    }
                } else {
                    eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Invalid id: {id_str}").as_str()));
                }
            } else {
                eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Note id is required."));
            }
        }).build()
}

pub fn build_search_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("search")
        .set_description("Search for a note by a query string")
        .set_optional(false)
        .add_argument("query")
        .add_option(
            &CliCommandOption {
                name: "tag".to_string(),
                short_name: Some("t".to_string()),
                description: Some("Narrow search to a tag".to_string()),
                is_flag: false
            }
        )
        .set_action(|args: HashMap<String, Vec<String>>| {
            let query = args.get("query").and_then(|v| v.last());
            let tags = args.get("tag");

            if query.is_none() && tags.is_none() {
                eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Query is required."));
            }

            let mut all_notes = notes::load_all_notes();

            // filter by tags
            if let Some(tags_list) = tags {
                all_notes = all_notes.into_iter().filter(|n| n.tags.iter().any(|t| tags_list.contains(t))).collect();
            }

            // filter by query
            if let Some(query_string) = query {
                all_notes = notes::slow_search(&all_notes, query_string)
            }

            if all_notes.is_empty() {
                println!("{}", print_utils::colorize(print_utils::Color::warning(), "No notes found."));
            } else {
                println!("Notes:");
                for note in all_notes {
                    println!("{}. {}", note.id, note.content);
                }
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
                        eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Invalid id: {id}").as_str()));
                        return;
                    }
                },
                None => {
                    eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Note id is required."));
                    return;
                }
            };

            let mut note = match notes::get_note_by_id(id) {
                Some(note) => note,
                None => {
                    eprintln!("{}", print_utils::colorize(print_utils::Color::warning(), format!("Note with id {id} not found.").as_str()));
                    return;
                }
            };

            let edited_note_content = if args.contains_key("interactive") || !args.contains_key("message") {
                match get_from_editor(Some(note.content)) {
                    Ok(content) => content,
                    Err(EditorOutputError) => {
                        if args.contains_key("message") {
                            args.get("message").and_then(|v| v.last()).unwrap_or(&String::new()).to_string()
                        } else {
                            return;
                        }
                    }
                }
            } else {
                args.get("message").and_then(|v| v.last()).unwrap_or(&String::new()).to_string()
            };

            note.content = edited_note_content.trim().to_string();
            notes::save_note(&note);
        }).build()
}

struct EditorOutputError;

fn get_from_editor(put_content: Option<String>) -> Result<String, EditorOutputError> {
    let config = notes::get_config();
    let editor = match config.editor {
        Some(e) => e,
        None => {
            eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "No editor available!"));
            return Err(EditorOutputError);
        }
    };
    
    // save note to temporary file
    let temp_file_path = "/tmp/rustic_note_tmp.txt".to_string();
    let mut file = match std::fs::File::create(&temp_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Error creating temporary file: {e}").as_str()));
            return Err(EditorOutputError);
        }
    };

    if put_content.is_some() {
        if let Err(e) = file.write_all(put_content.unwrap_or(String::new()).to_string().trim().as_bytes()) {
            eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Error writing note to temporary file: {e}").as_str()));
            return Err(EditorOutputError);
        }
    }

    std::process::Command::new(editor)
        .arg(&temp_file_path)
        .spawn()
        .expect(print_utils::colorize(print_utils::Color::error(), "Error: Failed to run editor").as_str())
        .wait()
        .expect(print_utils::colorize(print_utils::Color::error(), "Error: Editor returned a non-zero status").as_str());
    
    // read the edited note back
    match std::fs::read_to_string(&temp_file_path) {
        Ok(content) => Ok(content),
        Err(e) => {
            eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Error reading edited note: {e}").as_str()));
            return Err(EditorOutputError);
        }
    }
}
