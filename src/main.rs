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
                .set_description("Create a new note")
                .set_optional(false)
                .add_argument("note")
                .set_action(|args: HashMap<String, Vec<String>>| {
                    if let Some(note) = args.get("note") {
                        let note_content = note.last().unwrap_or(&String::from("")).to_string();
                        println!("Creating new note: {}", note_content);
                        let new_note = notes::RusticNote::new(note_content.clone());
                        notes::save_note(&new_note);
                    } else {
                        eprintln!("Error: Note name is required.");
                    }
                })
                .build(),
        )
        .build();
    cli.run(env::args());
}
