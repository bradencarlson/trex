use std::env;

mod parse_args;
mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_map = parse_args::parse(args);

    for (key,value) in &args_map {
        println!("{key}: {value}");
    }

    let file_arg = String::from("-f");
    let filename = match args_map.get(&file_arg) {
        Some(string) => string,
        None => ""
    };

    let config = config::read(filename);

    match config {
        Ok(string) => println!("config: {string}"),
        Err(e) => println!("Error: {e}"),
    };
}
