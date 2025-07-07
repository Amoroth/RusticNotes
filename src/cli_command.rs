use std::{collections::HashMap, env};

#[derive(Default)]
pub struct CliCommandBuilder {
    name: String,
    description: Option<String>,
    version: Option<String>,
    optional: bool,
    subcommands: Vec<CliCommand>,
    options: Vec<CliCommandOption>,
    action: Option<fn(HashMap<String, Vec<String>>)>,
}

impl CliCommandBuilder {
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
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

    pub fn set_optional(&mut self, optional: bool) -> &mut Self {
        self.optional = optional;
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

    pub fn set_action(&mut self, action: fn(HashMap<String, Vec<String>>)) -> &mut Self {
        self.action = Some(action.to_owned());
        self
    }

    pub fn build(&self) -> CliCommand {
        CliCommand {
            name: self.name.clone(),
            description: self.description.clone(),
            version: self.version.clone(),
            optional: self.optional.clone(),
            subcommands: self.subcommands.clone(),
            options: self.options.clone(),
            action: self.action.unwrap_or(|args: HashMap<String, Vec<String>>| {
                println!("Command executed with arguments: {:?}", args);
            }),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct CliCommand {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub optional: bool,
    pub subcommands: Vec<CliCommand>,
    pub options: Vec<CliCommandOption>,
    pub action: fn(HashMap<String, Vec<String>>),
}

impl CliCommand {
    pub fn run(&self, args: env::Args) {
        let env_args: Vec<String> = args.collect();
        let command = select_command(env_args.clone(), self);
        let arguments = collect_arguments(env_args, command);
        (command.action)(get_arguments_map(arguments));
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct CliCommandOption {
    pub name: String,
    pub short_name: Option<String>,
    pub is_flag: bool,
    pub optional: bool,
}

fn select_command(env_args: Vec<String>, command: &CliCommand) -> &CliCommand {
    if env_args.len() < 2 {
        return command;
    }

    let mut cmd = command;

    for arg in env_args.clone().into_iter().skip(1) {
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

    for arg in env_args.clone().into_iter().skip(1) {
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
            if previous_argument_definition.is_some() && !previous_argument_definition.unwrap().is_flag {
                args.last_mut().unwrap().1 = Some(arg.clone());
            }
        };
    }

    // check if all required arguments are present
    if !command.subcommands.is_empty() {
        for definition in &command.subcommands {
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

fn search_command<'a>(name: &str, command: &'a CliCommand) -> Option<&'a CliCommand> {
    if command.name == name {
        return Some(command);
    }

    for cmd in &command.subcommands {
        if cmd.name == name {
            return Some(cmd);
        }
    }

    None
}

fn search_command_options<'a>(name: &str, command: &'a CliCommand) -> Option<&'a CliCommandOption> {
    for option in &command.options {
        if option.name == name || option.short_name.as_deref() == Some(name) {
            return Some(option);
        }
    }

    None
}