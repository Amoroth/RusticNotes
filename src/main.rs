mod cli_parser;

use std::{collections::HashMap};
use cli_parser::{collect_arguments, CliConfigurable, CliArgument, CliArgumentSpecification};

#[derive(Debug)]
struct Arguments {
    pub name: CliArgument<String>,
    pub age: CliArgument<u32>,
}

impl CliConfigurable for Arguments {
    fn get_definitions(&mut self) -> Vec<CliArgumentSpecification> {
        vec![
            self.name.get_specification(false),
            self.age.get_specification(false)
        ]
    }

    fn populate(&mut self, args: &HashMap<String, String>) {
        self.name.set_value(&args);
        self.age.set_value(&args);
    }
}

fn main() {
    let mut arguments = Arguments {
        name: CliArgument::new("name".to_string(), Some("n".to_string())),
        age: CliArgument::new("age".to_string(), None),
    };
    collect_arguments(&mut arguments);

    println!("{:?}", arguments);
}
