mod cli_command;

use std::{collections::HashMap, env};
use cli_command::{CliCommand, CliCommandOption};

fn main() {
    // todo change struct to builder pattern
    let cmd = CliCommand {
        name: "cli_parser_example".to_string(),
        description: Some("An example CLI parser using Rust".to_string()),
        version: Some("0.1.0".to_string()),
        optional: false,
        options: vec![],
        action: |args: HashMap<String, Vec<String>>| {
            println!("root: {:?}", args);
        },
        subcommands: vec![
            CliCommand {
                name: "example".to_string(),
                description: Some("An example command to demonstrate CLI parsing".to_string()),
                version: Some("0.1.0".to_string()),
                optional: false,
                subcommands: vec![
                    CliCommand {
                        name: "new".to_string(),
                        description: Some("An examplen new command to demonstrate CLI parsing".to_string()),
                        version: Some("0.1.0".to_string()),
                        optional: false,
                        subcommands: vec![],
                        options: vec![
                            CliCommandOption {
                                name: "crap".to_string(),
                                short_name: Some("c".to_string()),
                                is_flag: false,
                                optional: true,
                            },
                        ],
                        action: |args: HashMap<String, Vec<String>>| {
                            println!("new: {:?}", args);
                        },
                    }
                ],
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
                    println!("example: {:?}", args);
                },
            }
        ],
    };
    cmd.run(env::args());
}
