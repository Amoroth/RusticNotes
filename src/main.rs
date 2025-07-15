mod cli_command;
mod notes;
mod note_commands;

use std::env;
use cli_command::{CliCommandBuilder, CliCommand};

const ROOT_VERSION: &str = "0.1.0";

fn main() {
    // todo add variadic positional argument
    let cli: CliCommand = CliCommandBuilder::default()
        .set_name("RusticNotes")
        .set_version(ROOT_VERSION)
        .set_description("A simplistic tool for managing notes")
        .add_subcommand(&note_commands::build_new_command())
        .add_subcommand(&note_commands::build_list_command())
        .add_subcommand(&note_commands::build_get_command())
        .add_subcommand(&note_commands::build_delete_command())
        .add_subcommand(&note_commands::build_search_command())
        .add_subcommand(&note_commands::build_edit_command())
        .build();
    cli.run(env::args());
}

// todo better error handling
// todo add tests
// todo save notes in markdown/org-mode files with json as a manifest/metadata
// todo edit note tags and others
// todo make opening editor optional in case you want to edit other properties?
// todo projects support and persistant switching between them
// todo active tui
