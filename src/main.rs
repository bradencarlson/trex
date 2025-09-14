use std::env;

pub mod parse_args;
mod config;
mod compile;
pub mod outline;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_map = parse_args::parse(args);

    let filename = match args_map.get(parse_args::FILENAME_ARG) {
        Some(string) => string,
        None => config::DEFAULT_CONFIG,
    };

    let outline = match config::read(filename) {
        Some(outline) => outline, 
        None => return,
    };

    println!("The outline I read was:\n\n{}\n", outline);

    match args_map.get(parse_args::COMPILE_ARG) {
        Some(_string) => {
            compile::compile(outline, &args_map);
        },
        None => {
            /* The default behavior is to NOT compile the document, this is to prevent 
             * firing up pdflatex every time I want to test something. In a
             * future version compiling will be the default. */
            println!("Pass the -c option in order to compile.");     
        },
    }
}
