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
    let config = match args_map.get(&file_arg) {
        Some(filename) => config::read(filename),
        None => {
            println!("No file passed to program, reading default.");
            config::read(config::DEFAULT_CONFIG)
        },
    };

}
