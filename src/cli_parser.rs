use std::{collections::HashMap, env, str::FromStr};

// todo leave only value in cliargument and use CliArgumentSpecification for
// the rest and include it in CliArgument as spec or something like that
#[derive(Debug)]
pub struct CliArgument<T: FromStr> { // todo check if I can make it without FromStr
    pub name: String,
    pub short_name: Option<String>, // optional short name for the argument
    pub value: Option<T>,
}

pub struct CliArgumentSpecification {
    pub name: String,
    pub short_name: Option<String>,
    pub is_flag: bool,
}

impl<T: FromStr> CliArgument<T> {
    pub fn new(name: String, short_name: Option<String>) -> Self {
        CliArgument {
            name,
            short_name,
            value: None,
        }
    }

    pub fn set_value(&mut self, args: &HashMap<String, String>) {        
        if let Some(value) = args.get(&self.name) {
            match value.parse::<T>() {
                Ok(parsed_value) => self.value = Some(parsed_value),
                Err(_) => eprintln!("Error parsing value for argument '{}': {}", self.name, value)
            }
        }
    }

    pub fn get_specification(&self, is_flag: bool) -> CliArgumentSpecification {
        CliArgumentSpecification {
            name: self.name.clone(),
            short_name: self.short_name.clone(),
            is_flag: is_flag,
        }
    }
}

pub trait CliConfigurable {
    fn get_definitions(&mut self) -> Vec<CliArgumentSpecification>;
    fn populate(&mut self, args: &HashMap<String, String>);
}

pub fn collect_arguments<T: CliConfigurable>(config: &mut T) {
    let env_args = env::args();
    let mut args: Vec<(String, Option<String>)> = vec![];
    let arugment_definitions = config.get_definitions();
    let mut previous_argument_definition: Option<&CliArgumentSpecification> = None;

    for arg in env_args.skip(1) {
        let argument_definition = if arg.starts_with("--") {
            let arg_key = arg.trim_start_matches("--").to_string();
            arugment_definitions.iter().find(|&x| x.name == arg_key)
        } else if arg.starts_with("-") {
            let arg_key = arg.trim_start_matches("-").to_string();
            arugment_definitions.iter().find(|&x| x.short_name.is_some() && x.short_name.as_ref().unwrap().to_string() == arg_key)
        } else {
            if previous_argument_definition.is_some() && !previous_argument_definition.unwrap().is_flag {
                args.last_mut().unwrap().1 = Some(arg.clone());
            }

            None
        };

        if argument_definition.is_some() {
            args.push((argument_definition.unwrap().name.clone(), None));
        }

        previous_argument_definition = argument_definition;
    }

    // x collect it all into the vector
    // the get_definitions method is used to check if the argument is positional or an option
    // check if the key is with a dash, if it is, add it to the vector as a tuple (key, None)
    // if it is not, add it to the vector as a tuple (key, Some(value))
    // take last value in the vector in populate method
    // this will also allow to have multiple values for the same key

    // don't bother with multiple values for now
    let mut args_hashmap: HashMap<String, String> = HashMap::new();
    for arg in args {
        args_hashmap.insert(arg.0, arg.1.unwrap_or_default());
    }
    
    config.populate(&args_hashmap);
}
