mod cli_command;
mod notes;

use std::{collections::HashMap, env};
use cli_command::{CliCommandBuilder, CliCommand, CliCommandOption};

const ROOT_VERSION: &str = "0.1.0";

fn main() {
    // todo add variadic positional argument
    let cli: CliCommand = CliCommandBuilder::default()
        .set_name("RusticNotes")
        .set_version(ROOT_VERSION)
        .set_description("A simplistic tool for managing notes")
        .add_subcommand(
            &CliCommandBuilder::default()
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
                }).build(),
        ).add_subcommand(
            &CliCommandBuilder::default()
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
                }).build(),
        ).add_subcommand(
            &CliCommandBuilder::default()
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
                }).build(),
        ).add_subcommand(
            &CliCommandBuilder::default()
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
                }).build(),
        ).add_subcommand(
            &CliCommandBuilder::default()
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
                }).build(),
        ).build();
    cli.run(env::args());
}

// todo better error handling
// todo add tests
// todo save notes in markdown/org-mode files with json as a manifest/metadata