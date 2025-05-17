use std::{collections::HashMap, env};

// todo accept structs as a generic?
// todo or make it a derive macro to implement collecting arguments into it

pub fn collect_arguments() -> HashMap<String, String> {
    let arguments = env::args();
    let mut args_hashmap: HashMap<String, String> = HashMap::new();

    let mut previous_argument: String = "".to_string();

    for arg in arguments.skip(1) {
        if arg.contains("=") {
            // todo write a function to separate them and clean them up.
            // key should be checked if it is a short argument or a long argument
            // key should be cleaned of leading '-' for a short argument or '--' for a long argument
            let arg_pair: Vec<String> = arg
                .split('=')
                .map(|x| x.into())
                .collect::<Vec<String>>()
                .clone();
            let key = arg_pair.get(0).unwrap_or(&"".to_string()).to_string();
            let value = arg_pair.get(1).unwrap_or(&"".to_string()).to_string();
            args_hashmap.insert(key, value);
        } else {
            if previous_argument.is_empty() {
                previous_argument = arg;
            } else {
                args_hashmap.insert(previous_argument, arg);
                previous_argument = "".to_string()
            }
        }
    }

    args_hashmap
}

// type `&'static str`
// & - borrowed reference
// 'static - lifetime notation for something. static is a special case and means for the whole lifetime of the program
// str - is just a sequence of bytes
// &'static means that the value is a borrowed 'dangling' reference to a sequence of bytes and CANNOT be changed, since its shared
