use std::{collections::HashMap, env};

use serde::de;

type CliCommandAction = fn(HashMap<String, Vec<String>>);

#[derive(Default)]
pub struct CliCommandBuilder {
    name: String,
    aliases: Vec<String>,
    description: Option<String>,
    version: Option<String>,
    optional: bool,
    arguments: Vec<String>,
    subcommands: Vec<CliCommand>,
    options: Vec<CliCommandOption>,
    action: Option<CliCommandAction>,
}

impl CliCommandBuilder {
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn add_alias(&mut self, alias: &str) -> &mut Self {
        self.aliases.push(alias.to_string());
        self
    }

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn set_version(&mut self, version: &str) -> &mut Self {
        self.version = Some(version.to_string());
        self
    }

    pub fn add_argument(&mut self, argument: &str) -> &mut Self {
        self.arguments.push(argument.to_string());
        self
    }

    pub fn add_subcommand(&mut self, subcommand: &CliCommand) -> &mut Self {
        self.subcommands.push(subcommand.clone());
        self
    }

    pub fn add_option(&mut self, option: &CliCommandOption) -> &mut Self {
        self.options.push(option.clone());
        self
    }

    pub fn set_action(&mut self, action: CliCommandAction) -> &mut Self {
        self.action = Some(action.to_owned());
        self
    }

    pub fn build(&self) -> CliCommand {
        CliCommand {
            name: self.name.clone(),
            aliases: self.aliases.clone(),
            description: self.description.clone(),
            version: self.version.clone(),
            arguments: self.arguments.clone(),
            subcommands: self.subcommands.clone(),
            options: self.options.clone(),
            action: self.action,
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct CliCommand {
    pub name: String,
    pub aliases: Vec<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub arguments: Vec<String>,
    pub subcommands: Vec<CliCommand>,
    pub options: Vec<CliCommandOption>,
    pub action: Option<CliCommandAction>,
}

impl CliCommand {
    // todo #939 if command is provided but not found, print error and ask user to use --help
    pub fn run(&self, args: env::Args) {
        let env_args: Vec<String> = args.skip(1).collect();
        let command = select_command(env_args.clone(), self);

        if search_for_help_flag(env_args.clone()) {
            self.get_version();
            command.get_help();
            return;
        }

        if search_for_version_flag(env_args.clone()) {
            self.get_version();
            return;
        }

        // remove arguments that choose a subcommand
        let command_index = env_args.iter().position(|arg| *arg == command.name).unwrap_or(0);
        let env_args: Vec<String> = env_args.into_iter().enumerate()
            .filter(|(index, arg)| arg.starts_with("-") || *index > command_index)
            .map(|(_, arg)| arg)
            .collect();

        let arguments = collect_arguments(env_args, command);

        if command.action.is_some() {
            (command.action.unwrap())(get_arguments_map(arguments));
        } else {
            command.get_help();
        }
    }

    pub fn get_help(&self) {
        let padding_width = 4;

        self.print_help_description(padding_width);
        self.print_help_usage(padding_width);
        self.print_help_example(padding_width);
        self.print_help_subcommands(padding_width);
        self.print_help_options(padding_width);
    }

    fn print_help_description(&self, padding_width: usize) {
        if let Some(description) = &self.description {
            println!();
            println!("DESCRIPTION");
            println!("{padding}{description}", padding = " ".repeat(padding_width));
        }
    }

    fn print_help_usage(&self, padding_width: usize) {
        println!();
        println!("USAGE");
        println!("{padding}$ {0}{1}{2}", self.name, if self.subcommands.is_empty() { "" } else if self.action.is_none() { " [COMMAND]" } else { " COMMAND" }, if self.options.is_empty() { "" } else { " [OPTIONS]" }, padding = " ".repeat(padding_width));
    }

    fn print_help_example(&self, padding_width: usize) {
        if false {
            println!();
            println!("EXAMPLE");
            println!("{padding}{}", self.name, padding = " ".repeat(padding_width)); // placeholder, self.example
        }
    }

    fn print_help_subcommands(&self, padding_width: usize) {
        if self.subcommands.is_empty() {
            return;
        }

        println!();
        println!("COMMANDS");

        let display_items: Vec<(String, &str)> = self.subcommands
            .iter()
            .map(|subcmd| {
                let name = subcmd.name.clone();
                let description = subcmd.description.as_deref().unwrap_or("");
                (name, description)
            })
            .collect();

        let longest_name_len = display_items
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(0);

        let padding = " ".repeat(padding_width);
        for (name, description) in &display_items {
            println!("{padding}{name:<longest_name_len$} - {description}");
        }
        println!();
        println!("{padding}Use \"{} COMMAND --help\" for more information about a command.", self.name, padding = " ".repeat(padding_width));
    }

    fn print_help_options(&self, padding_width: usize) {
        if self.options.is_empty() {
            return;
        }

        println!();
        println!("OPTIONS");

        let display_items: Vec<(String, &str)> = self.options
            .iter()
            .map(|option| {
                let short_name = option.short_name.as_ref().map_or(String::new(), |s| format!("-{s}, "));
                let name = if option.is_flag {
                    format!("{short_name}--{}, ", option.name)
                } else {
                    format!("{short_name}--{} <value>, ", option.name)
                };
                let description = option.description.as_deref().unwrap_or("");
                (name, description)
            })
            .collect();

        let longest_name_len = display_items
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(0);

        let padding = " ".repeat(padding_width);
        for (name, description) in &display_items {
            println!("{padding}{name:<longest_name_len$}{description}");
        }
    }

    pub fn get_version(&self) {
        println!("{} {}", self.name, self.version.as_ref().unwrap_or(&String::from("")));
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct CliCommandOption {
    pub name: String,
    pub short_name: Option<String>,
    pub is_flag: bool,
    pub description: Option<String>,
}

fn select_command(env_args: Vec<String>, command: &CliCommand) -> &CliCommand {
    if env_args.is_empty() {
        return command;
    }

    let mut cmd = command;

    for arg in env_args.clone() {
        if !arg.starts_with("-") {
            if let Some(subcommand) = search_command(&arg, cmd) {
                cmd = subcommand;
            }
        }
    }

    // If no subcommand matches, return the root command
    cmd
}

fn collect_arguments(env_args: Vec<String>, command: &CliCommand) -> Vec<(String, Option<String>)> {
    let mut args: Vec<(String, Option<String>)> = vec![];
    let mut previous_argument_definition: Option<&CliCommandOption> = None;
    let mut positional_index = 0;

    for arg in env_args.clone() {
        if arg.starts_with("--") {
            let arg_key = arg.trim_start_matches("--").to_string();
            let option_definition = search_command_options(arg_key.as_str(), command);
            if option_definition.is_some() {
                args.push((option_definition.unwrap().name.clone(), if option_definition.unwrap().is_flag { Some(String::from("true")) } else { None }));
                previous_argument_definition = option_definition;
            }
        } else if arg.starts_with("-") {
            let arg_key = arg.trim_start_matches("-").to_string();
            let option_definition = search_command_options(arg_key.as_str(), command);
            if option_definition.is_some() {
                args.push((option_definition.unwrap().name.clone(), if option_definition.unwrap().is_flag { Some(String::from("true")) } else { None }));
                previous_argument_definition = option_definition;
            }
        } else {
            if previous_argument_definition.is_none() {
                let argument_definition = command.arguments.get(positional_index);
                if let Some(argument_definition) = argument_definition {
                    args.push((argument_definition.clone(), Some(arg.clone())));
                    positional_index += 1;
                }
            }

            if previous_argument_definition.is_some() && !previous_argument_definition.unwrap().is_flag {
                args.last_mut().unwrap().1 = Some(arg.clone());
                previous_argument_definition = None;
            }
        };
    }

    args
}

fn get_arguments_map(arguments: Vec<(String, Option<String>)>) -> HashMap<String, Vec<String>> {
    let mut args_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    for arg in arguments {
        args_hashmap.entry(arg.0).or_default().push(arg.1.unwrap_or_default());
    }
    args_hashmap
}

fn search_command<'a>(name: &str, command: &'a CliCommand) -> Option<&'a CliCommand> {
    if command.name == name {
        return Some(command);
    }

    command.subcommands.iter().find(|&cmd| cmd.name == name || cmd.aliases.contains(&name.to_string()))
}

fn search_command_options<'a>(name: &str, command: &'a CliCommand) -> Option<&'a CliCommandOption> {
    command.options.iter().find(|&option| option.name == name || option.short_name.as_deref() == Some(name))
}

fn search_for_help_flag(env_args: Vec<String>) -> bool {
    env_args.iter().any(|arg| arg == "--help" || arg == "-h")
}

fn search_for_version_flag(env_args: Vec<String>) -> bool {
    env_args.iter().any(|arg| arg == "--version" || arg == "-V")
}
