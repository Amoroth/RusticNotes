mod cli_parser;

use cli_parser::collect_arguments;

fn main() {
    let arguments = collect_arguments();

    println!("{:?}", arguments);
}
