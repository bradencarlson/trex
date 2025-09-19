use std::collections::HashMap;

pub mod parse_args;
pub mod config;
pub mod outline;
pub mod compile;

pub fn run(args: &HashMap<String,String>) {

    let filename = match args.get(parse_args::FILENAME_ARG) {
        Some(string) => string,
        None => config::DEFAULT_CONFIG,
    };

    let outline = match config::read(filename) {
        Some(outline) => outline, 
        None => return,
    };

    println!("The outline I read was:\n\n{}\n", outline);

    match args.get(parse_args::COMPILE_ARG) {
        Some(_string) => {
            compile::compile(outline, &args);
        },
        None => {
            /* The default behavior is to NOT compile the document, this is to prevent 
             * firing up pdflatex every time I want to test something. In a
             * future version compiling will be the default. */
            println!("Pass the -c option in order to compile.");     
        },
    }

}
