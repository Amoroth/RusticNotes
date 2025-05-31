use std::{collections::HashMap, env, convert::From};

#[derive(Debug)]
pub struct CliArgument<T: From<String>> { // todo check if I can make it without From<String>
    pub name: String,
    pub value: Option<T>,
}

impl<T: From<String>> CliArgument<T> {
    pub fn new(name: String) -> Self {
        CliArgument {
            name,
            value: None,
        }
    }

    pub fn set_value(&mut self, args: &HashMap<String, String>) {        
        if let Some(value) = args.get(&self.name) {
            self.value = Some(value.clone().into());
        }
    }
}

pub trait CliConfigurable {
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

    config.populate(&args_hashmap);
}
