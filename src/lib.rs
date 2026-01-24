/* lib.rs
 *
 * Author: Braden Carlson
 * Date: September 2025
 *
 */

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

    if let Some(_s) = args.get(parse_args::CLEAN_ARG) {
        compile::clean(outline);
        return
    }

    if let Some(_s) = args.get(parse_args::VERBOSE_ARG) {
        println!("The outline I read was:\n\n{}\n", outline);
    }

    compile::compile(outline, &args);
}
