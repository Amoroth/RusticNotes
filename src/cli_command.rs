use std::{collections::HashMap, env};

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

    pub fn set_optional(&mut self, optional: bool) -> &mut Self {
        self.optional = optional;
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

    pub fn set_action(&mut self, action: fn(HashMap<String, Vec<String>>)) -> &mut Self {
        self.action = Some(action.to_owned());
        self
    }

    pub fn build(&self) -> CliCommand {
        CliCommand {
            name: self.name.clone(),
            aliases: self.aliases.clone(),
            description: self.description.clone(),
            version: self.version.clone(),
            optional: self.optional,
            arguments: self.arguments.clone(),
            subcommands: self.subcommands.clone(),
            options: self.options.clone(),
            action: self.action.unwrap_or(|args: HashMap<String, Vec<String>>| {
                println!("Command executed with arguments: {args:?}");
            }),
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
    pub optional: bool,
    pub arguments: Vec<String>,
    pub subcommands: Vec<CliCommand>,
    pub options: Vec<CliCommandOption>,
    pub action: fn(HashMap<String, Vec<String>>),
}

impl CliCommand {
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

        // todo if argument is after options, it is not detected (?)
        // remove arguments that choose a subcommand
        let command_index = env_args.iter().position(|arg| *arg == command.name).unwrap_or(0);
        let env_args: Vec<String> = env_args.into_iter().enumerate()
            .filter(|(index, arg)| arg.starts_with("-") || *index > command_index)
            .map(|(_, arg)| arg)
            .collect();

        let arguments = collect_arguments(env_args, command);
        (command.action)(get_arguments_map(arguments));
    }

    pub fn get_help(&self) {
        if let Some(description) = &self.description {
            println!();
            println!("DESCRIPTION");
            println!("    {description}"); // todo dynamic padding
        }

        println!();
        println!("USAGE");
        // todo if root can be called without commands, add [] to COMMAND
        println!("    $ {}{}{}", self.name, if self.subcommands.is_empty() { "" } else { " COMMAND" }, if self.options.is_empty() { "" } else { " [OPTIONS]" });

        if false {
            println!();
            println!("EXAMPLE");
            println!("    {}", self.name); // placeholder, self.example
        }

        if !self.subcommands.is_empty() {
            println!();
            println!("COMMANDS");
            for subcommand in &self.subcommands {
                let cmd_name = if subcommand.optional { format!("[{}]", subcommand.name) } else { subcommand.name.to_string() };
                println!("    {} - {}", cmd_name, subcommand.description.as_deref().unwrap_or(""));
            }
            println!();
            // todo if root can be called without commands, add [] to COMMAND
            println!("    Use \"{} COMMAND --help\" for more information about a command.", self.name);
        }

        if !self.options.is_empty() {
            println!();
            println!("OPTIONS");
            // todo dynamic padding between names and descriptions
            for option in &self.options {
                let name = if option.is_flag {
                    format!("--{}", option.name)
                } else {
                    format!("--{} <value>", option.name)
                };
                let short_name = option.short_name.as_ref().map_or(String::new(), |s| format!("-{s}, "));
                println!("    {}{}, {}", short_name, name, option.description.as_deref().unwrap_or(""));
            }
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

    for (index, arg) in env_args.clone().into_iter().enumerate() {
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
                let argument_definition = command.arguments.get(index);
                if let Some(argument_definition) = argument_definition {
                    args.push((argument_definition.clone(), Some(arg.clone())));
                }
            }

            if previous_argument_definition.is_some() && !previous_argument_definition.unwrap().is_flag {
                args.last_mut().unwrap().1 = Some(arg.clone());
                previous_argument_definition = None;
            }
        };
    }

    // check if all required arguments are present
    // bug if a required subcommand is not present, it will display name of the first subcommand instead
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
