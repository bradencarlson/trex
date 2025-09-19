use std::env;

mod parse_args;

use trex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_map = parse_args::parse(args);

    trex::run(&args_map)
}
