mod cli_parser;

use std::{collections::HashMap};
use cli_parser::{collect_arguments, CliConfigurable, CliArgument, CliArgumentSpecification};

use crate::cli_parser::CliArgumentParsable;

#[derive(Debug)]
enum ActionEnum {
    Create,
    Update,
    Delete,
}

impl CliArgumentParsable for ActionEnum {
    fn parse_argument(value: &[String]) -> Result<Self, ()> {
        if value.is_empty() {
            Err(())
        } else {
            match value.first().unwrap().to_lowercase().as_str() {
                "create" => Ok(ActionEnum::Create),
                "update" => Ok(ActionEnum::Update),
                "delete" => Ok(ActionEnum::Delete),
                _ => Err(()),
            }
        }
    }
}

#[derive(Debug)]
struct Arguments {
    pub action: CliArgument<ActionEnum>,
    pub name: CliArgument<String>,
    pub age: CliArgument<u32>,
    pub adult: CliArgument<bool>,
    pub personalities: CliArgument<Vec<String>>,
}

impl CliConfigurable for Arguments {
    fn get_definitions(&mut self) -> Vec<&CliArgumentSpecification> {
        vec![
            &self.action.specification,
            &self.name.specification,
            &self.age.specification,
            &self.adult.specification,
            &self.personalities.specification,
        ]
    }

    fn populate(&mut self, args: &HashMap<String, Vec<String>>) {
        self.action.set_value(args.get(&self.action.specification.name).unwrap_or(&vec![]));
        self.name.set_value(args.get(&self.name.specification.name).unwrap_or(&vec![]));
        self.age.set_value(args.get(&self.age.specification.name).unwrap_or(&vec![]));
        self.adult.set_value(args.get(&self.adult.specification.name).unwrap_or(&vec![]));
        self.personalities.set_value(args.get(&self.personalities.specification.name).unwrap_or(&vec![]));
    }
}

fn main() {
    let mut arguments = Arguments {
        action: CliArgument::new(CliArgumentSpecification {
            name: "action".to_string(),
            short_name: None,
            is_flag: false,
            optional: false,
        }),
        name: CliArgument::new(CliArgumentSpecification {
            name: "name".to_string(),
            short_name: Some("n".to_string()),
            is_flag: false,
            optional: true,
        }),
        age: CliArgument::new(CliArgumentSpecification {
            name: "age".to_string(),
            short_name: None,
            is_flag: false,
            optional: true,
        }),
        adult: CliArgument::new(CliArgumentSpecification {
            name: "adult".to_string(),
            short_name: Some("a".to_string()),
            is_flag: true,
            optional: true,
        }),
        personalities: CliArgument::new(CliArgumentSpecification {
            name: "personalities".to_string(),
            short_name: Some("p".to_string()),
            is_flag: false,
            optional: true,
        }),
    };
    collect_arguments(&mut arguments);

    println!("{:?}", arguments);
}
