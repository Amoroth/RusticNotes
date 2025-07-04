use std::{collections::HashMap, env};

pub trait CliArgumentParsable {
    fn parse_argument(value: &[String]) -> Result<Self, ()> where Self: Sized;
}

impl CliArgumentParsable for String {
    fn parse_argument(value: &[String]) -> Result<Self, ()> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(value.join(" "))
        }
    }
}

impl CliArgumentParsable for u32 {
    fn parse_argument(value: &[String]) -> Result<Self, ()> {
        if value.is_empty() {
            Err(())
        } else {
            value[0].parse::<u32>().map_err(|_| ())
        }
    }
}

impl CliArgumentParsable for bool {
    fn parse_argument(value: &[String]) -> Result<Self, ()> {
        if value.is_empty() {
            Err(())
        } else {
            match value[0].to_lowercase().as_str() {
                "true" | "1" => Ok(true),
                "false" | "0" => Ok(false),
                _ => Err(()),
            }
        }
    }
}

#[derive(Debug)]
pub struct CliArgument<T: CliArgumentParsable> {
    pub specification: CliArgumentSpecification,
    pub value: Option<T>,
}

#[derive(Debug)]
pub struct CliArgumentSpecification {
    pub name: String,
    pub short_name: Option<String>, // optional short name for the argument
    pub is_flag: bool,
}

impl<T: CliArgumentParsable> CliArgument<T> {
    pub fn new(specification: CliArgumentSpecification) -> Self {
        CliArgument {
            value: None,
            specification,
        }
    }

    pub fn set_value(&mut self, args: &Vec<String>) {
        self.value = match T::parse_argument(args) {
            Ok(parsed_value) => Some(parsed_value),
            Err(_) => {
                eprintln!("Error parsing value for argument '{}': {:?}", self.specification.name, args);
                None
            }
        };
    }
}

pub trait CliConfigurable {
    fn get_definitions(&mut self) -> Vec<&CliArgumentSpecification>;
    fn populate(&mut self, args: &HashMap<String, Vec<String>>);
}

pub fn collect_arguments<T: CliConfigurable>(config: &mut T) {
    let env_args = env::args();
    let mut args: Vec<(String, Option<String>)> = vec![];
    let arugment_definitions = config.get_definitions();
    let mut previous_argument_definition: Option<&&CliArgumentSpecification> = None;

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
            args.push((argument_definition.unwrap().name.clone(), if argument_definition.unwrap().is_flag { Some(String::from("true")) } else { None }));
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
    let mut args_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    for arg in args {
        args_hashmap.insert(arg.0, vec![arg.1.unwrap_or_default()]);
    }
    
    config.populate(&args_hashmap);
}
