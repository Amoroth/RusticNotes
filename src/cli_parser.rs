use std::{collections::HashMap, env, str::FromStr};

#[derive(Debug)]
pub struct CliArgument<T: FromStr> { // todo check if I can make it without FromStr
    pub name: String,
    pub value: Option<T>,
}

pub struct CliArgumentSpecification {
    pub name: String,
    pub is_flag: bool,
}

impl<T: FromStr> CliArgument<T> {
    pub fn new(name: String) -> Self {
        CliArgument {
            name,
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

    pub fn get_specification(&self) -> CliArgumentSpecification {
        CliArgumentSpecification {
            name: self.name.clone(),
            is_flag: self.value.is_none(), // todo: check this from the generic type T if possible
        }
    }
}

pub trait CliConfigurable {
    fn get_definitions(&mut self) -> Vec<CliArgumentSpecification>;
    fn populate(&mut self, args: &HashMap<String, String>);
}

pub fn collect_arguments<T: CliConfigurable>(config: &mut T) {
    let arguments = env::args();
    let mut args_hashmap = HashMap::new();
    let mut previous_argument_key: Option<String> = None;

    for arg in arguments.skip(1) {
        if let Some(prev_key) = previous_argument_key.take() {
            args_hashmap.insert(prev_key, arg);
        } else if let Some((key, value)) = arg.split_once('=') {
            args_hashmap.insert(key.to_string(), value.to_string());
        } else {
            previous_argument_key = Some(arg);
        }
    }

    // collect it all into the vector
    // the get_definitions method is used to check if the argument is positional or an option
    // check if the key is with a dash, if it is, add it to the vector as a tuple (key, None)
    // if it is not, add it to the vector as a tuple (key, Some(value))
    // take last value in the vector in populate method
    // this will also allow to have multiple values for the same key

    config.populate(&args_hashmap);
}
