use std::env;

pub mod parse_args;
mod config;
mod compile;
pub mod outline;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_map = parse_args::parse(args);

    let file_arg = String::from("-f");

    let filename = match args_map.get(&file_arg) {
        Some(string) => string,
        None => config::DEFAULT_CONFIG,
    };

    let outline = match config::read(filename) {
        Some(outline) => outline, 
        None => return,
    };

    println!("{}", outline);

    compile::compile(outline, &args_map);

}
