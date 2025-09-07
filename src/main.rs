use std::env;
use std::fs;

mod parse_args;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = args.get(1);
    match arg {
        Some(arg) => println!("arg: {arg}"),
        None => println!("No arguments passed."),
    }

    let content = fs::read_to_string("test_file.txt");
    match content {
        Ok(string) => println!("Content of file: {string}"),
        Err(_) => println!("Something went wrong while reading the file."),
    }
}
