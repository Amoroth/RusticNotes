mod cli_command;

use std::{collections::HashMap, env};
use cli_command::{CliCommandBuilder, CliCommandOption};

fn main() {
    let cmd = CliCommandBuilder::default()
        .set_name("cli_parser_example")
        .set_description("An example CLI parser using Rust")
        .set_version("0.1.0")
        .set_optional(false)
        .add_subcommand(
            &CliCommandBuilder::default()
                .set_name("example")
                .set_description("An example command to demonstrate CLI parsing")
                .set_version("0.1.0")
                .set_optional(true)
                .add_option(
                    &CliCommandOption {
                        name: "name".to_string(),
                        short_name: Some("n".to_string()),
                        is_flag: false,
                        optional: true,
                        description: Some("The name of the person".to_string()),
                    })
                .add_option(
                    &CliCommandOption {
                        name: "age".to_string(),
                        short_name: None,
                        is_flag: false,
                        optional: true,
                        description: Some("The age of the person".to_string()),
                    })
                .add_option(
                    &CliCommandOption {
                        name: "adult".to_string(),
                        short_name: Some("a".to_string()),
                        is_flag: true,
                        optional: true,
                        description: Some("Whether the person is an adult".to_string()),
                    })
                .add_option(
                    &CliCommandOption {
                        name: "personalities".to_string(),
                        short_name: Some("p".to_string()),
                        is_flag: false,
                        optional: true,
                        description: Some("A list of personalities".to_string()),
                    })
                .add_subcommand(
                    &CliCommandBuilder::default()
                        .set_name("new")
                        .set_description("An example new command to demonstrate CLI parsing")
                        .set_version("0.1.0")
                        .set_optional(true)
                        .add_argument("note")
                        .add_option(
                            &CliCommandOption {
                                name: "crap".to_string(),
                                short_name: Some("c".to_string()),
                                is_flag: false,
                                optional: true,
                                description: Some("A crap option".to_string()),
                            })
                        .set_action(|args: HashMap<String, Vec<String>>| {
                            println!("new: {:?}", args);
                        })
                        .build(),
                )
                .set_action(|args: HashMap<String, Vec<String>>| {
                    println!("example: {:?}", args);
                })
                .build()
        )
        .set_action(|args: HashMap<String, Vec<String>>| {
            println!("root: {:?}", args);
        })
        .build();
    // todo implement short name chaining
    cmd.run(env::args());
}
