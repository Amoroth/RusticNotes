mod cli_parser;

use std::{collections::HashMap};
use cli_parser::{collect_arguments, Configurable};

#[derive(Debug)]
struct Arguments {
    pub name: String,
}

impl Configurable for Arguments {
    fn populate(&mut self, args: &HashMap<String, String>) {
        if let Some(name) = args.get("name") {
            self.name = name.clone();
        }
    }
}

fn main() {
    let mut arguments = Arguments {
        name: String::new(),
    };
    collect_arguments(&mut arguments);

    println!("{:?}", arguments);
}
