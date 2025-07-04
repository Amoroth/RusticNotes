use std::{collections::HashMap, env};

#[derive(Debug)]
pub struct CliCommand {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub subcommands: Vec<CliCommand>,
    pub optional: bool,
    pub options: Vec<CliCommandOption>,
    pub action: fn(HashMap<String, Vec<String>>),
}

impl CliCommand {
    pub fn run(&self, args: env::Args) {
        let arguments = collect_arguments(args, self);
        (self.action)(get_arguments_map(arguments));
    }
}

#[derive(Debug)]
pub struct CliCommandOption {
    pub name: String,
    pub short_name: Option<String>,
    pub is_flag: bool,
    pub optional: bool,
}

pub fn collect_arguments(env_args: env::Args, command: &CliCommand) -> Vec<(String, Option<String>)> {
    let mut args: Vec<(String, Option<String>)> = vec![];
    let mut previous_argument_definition: Option<&CliCommandOption> = None;

    for (index, arg) in env_args.skip(1).enumerate() {
        if arg.starts_with("--") {
            let arg_key = arg.trim_start_matches("--").to_string();
            let option_definition = search_command_options(arg_key.as_str(), command);
            args.push((option_definition.unwrap().name.clone(), if option_definition.unwrap().is_flag { Some(String::from("true")) } else { None }));
            previous_argument_definition = option_definition;
        } else if arg.starts_with("-") {
            let arg_key = arg.trim_start_matches("-").to_string();
            let option_definition = search_command_options(arg_key.as_str(), command);
            args.push((option_definition.unwrap().name.clone(), if option_definition.unwrap().is_flag { Some(String::from("true")) } else { None }));
            previous_argument_definition = option_definition;
        } else {
            // check if argument could be a positional argument by comparing its value with enum
            if previous_argument_definition.is_none() {
                let command_definition = search_command(&arg, command);
                
                if command_definition.is_some() {
                    return collect_arguments(env_args, command)
                }
            } else {
                if previous_argument_definition.is_some() && !previous_argument_definition.unwrap().is_flag {
                    args.last_mut().unwrap().1 = Some(arg.clone());
                }
            }
        };
    }

    // check if all required arguments are present
    if !command.subcommands.is_empty() {
        for definition in command.subcommands {
            if !definition.optional && !args.iter().any(|(name, _)| name == &definition.name) {
                eprintln!("Missing required argument: {}", definition.name);
                std::process::exit(1);
            }
        }
    }

    args
}

fn get_arguments_map(arguments: Vec<(String, Option<String>)>) -> HashMap<String, Vec<String>> {
    let mut args_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    for arg in arguments {
        args_hashmap.entry(arg.0).or_insert(Vec::new()).push(arg.1.unwrap_or_default());
    }
    args_hashmap
}

fn search_command(name: &str, command: &CliCommand) -> Option<&CliCommand> {
    if command.name == name {
        return Some(command);
    }

    if !command.subcommands.is_empty() {
        for cmd in command.subcommands {
            if cmd.name == name {
                return Some(cmd);
            }
            if let Some(subcommand) = search_command(name, &cmd.subcommands) {
                return Some(subcommand);
            }
        }
    }

    None
}

fn search_command_options<'a>(name: &str, command: &CliCommand) -> Option<&'a CliCommandOption> {
    for option in &command.options {
        if option.name == name || option.short_name.as_deref() == Some(name) {
            return Some(option);
        }
    }

    for subcommand in &command.subcommands {
        if let Some(option) = search_command_options(name, subcommand) {
            return Some(option);
        }
    }

    None
}