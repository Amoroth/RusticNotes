use std::{collections::HashMap, env};

pub trait CliArgumentParsable {
    fn parse_argument(value: &[String]) -> Result<Self, ()> where Self: Sized;
}

impl CliArgumentParsable for String {
    fn parse_argument(value: &[String]) -> Result<Self, ()> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(value.last().unwrap_or(&String::new()).clone())
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

impl<T: CliArgumentParsable> CliArgumentParsable for Vec<T> {
    fn parse_argument(value: &[String]) -> Result<Self, ()> {
        if value.is_empty() {
            Err(())
        } else {
            let mut parsed_values = Vec::new();
            for v in value {
                match T::parse_argument(&[v.clone()]) {
                    Ok(parsed_value) => parsed_values.push(parsed_value),
                    Err(_) => return Err(()),
                }
            }
            Ok(parsed_values)
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
    pub short_name: Option<String>,
    pub is_flag: bool,
    pub optional: bool,
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

    for (index, arg) in env_args.skip(1).enumerate() {
        let argument_definition = if arg.starts_with("--") {
            let arg_key = arg.trim_start_matches("--").to_string();
            arugment_definitions.iter().find(|&x| x.name == arg_key)
        } else if arg.starts_with("-") {
            let arg_key = arg.trim_start_matches("-").to_string();
            arugment_definitions.iter().find(|&x| x.short_name.is_some() && x.short_name.as_ref().unwrap().to_string() == arg_key)
        } else {
            if previous_argument_definition.is_none() {
                arugment_definitions.get(index)
            } else {
                if previous_argument_definition.is_some() && !previous_argument_definition.unwrap().is_flag {
                    args.last_mut().unwrap().1 = Some(arg.clone());
                }
    
                None
            }
        };

        if argument_definition.is_some() {
            args.push((argument_definition.unwrap().name.clone(), if argument_definition.unwrap().is_flag { Some(String::from("true")) } else { None }));
        }

        previous_argument_definition = argument_definition;
    }

    // check if all required arguments are present
    for definition in arugment_definitions {
        if !definition.optional && !args.iter().any(|(name, _)| name == &definition.name) {
            eprintln!("Missing required argument: {}", definition.name);
            std::process::exit(1);
        }
    }

    let mut args_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    for arg in args {
        args_hashmap.entry(arg.0).or_insert(Vec::new()).push(arg.1.unwrap_or_default());
    }
    
    config.populate(&args_hashmap);
}
