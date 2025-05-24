use std::{collections::HashMap, env};

// todo accept structs as a generic?
// todo or make it a derive macro to implement collecting arguments into it

pub fn collect_arguments() -> HashMap<String, String> {
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

    args_hashmap
}

// type `&'static str`
// & - borrowed reference
// 'static - lifetime notation for something. static is a special case and means for the whole lifetime of the program
// str - is just a sequence of bytes
// &'static means that the value is a borrowed 'dangling' reference to a sequence of bytes and CANNOT be changed, since its shared
