mod cli_command;

use std::{collections::HashMap, env};
use cli_command::{CliCommand, CliCommandOption};

fn main() {
    let cmd = CliCommand {
        name: "cli_parser_example".to_string(),
        description: "An example CLI parser using Rust".to_string(),
        version: "0.1.0".to_string(),
        subcommands: vec![
            CliCommand {
                name: "example".to_string(),
                description: "An example command to demonstrate CLI parsing".to_string(),
                options: vec![
                    CliCommandOption {
                        name: "name".to_string(),
                        short_name: Some("n".to_string()),
                        is_flag: false,
                        optional: true,
                    },
                    CliCommandOption {
                        name: "age".to_string(),
                        short_name: None,
                        is_flag: false,
                        optional: true,
                    },
                    CliCommandOption {
                        name: "adult".to_string(),
                        short_name: Some("a".to_string()),
                        is_flag: true,
                        optional: true,
                    },
                    CliCommandOption {
                        name: "personalities".to_string(),
                        short_name: Some("p".to_string()),
                        is_flag: false,
                        optional: true,
                    },
                ],
                action: |args: HashMap<String, Vec<String>>| {
                    println!("{:?}", args);
                },
            }
        ],
    };
    cmd.run(env::args());
}
