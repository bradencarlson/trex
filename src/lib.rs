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

    if let Some(_s) = args.get(parse_args::VERBOSE_ARG) {
        println!("The outline I read was:\n\n{}\n", outline);
    }

    /* If the dryrun argument was found, do nothing */
    match args.get(parse_args::DRYRUN_ARG) {
        Some(_string) => (),
        None => {
            compile::compile(outline, &args);
        },
    }

}
