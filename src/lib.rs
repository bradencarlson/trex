/* lib.rs
 *
 * Author: Braden Carlson
 * Date: September 2025
 *
 */

use std::collections::HashMap;

pub mod parse_args;
pub mod config;
pub mod compile;
pub mod utils;

const HELP_MESSAGE: &str = "\
TreX Help Page\n\
\n\
Compiles multi-file markup languages. Currently, only LaTeX is supported.
TreX reads the structure of your document from a configuration file (default
location is ./trex.conf), then runs pdflatex with the appropriate options.\n
Command line options: \n
  -c  Clean up. Removes all auxiliary files created when compiling.\n
  -d  Perform a dry run, do not compile.\n
  -e  Select an engine, default is 'pdflatex'.\n
  -f  Specify a config file other than the default: './trex.conf'.\n
  -h  Show this help page.\n
  -j  Specify jobname (name of output file, do not include file extension).\n
  -o  Specify a class option.\n
  -q  Suppress all output.\n
  -r  Specify a range of files. This can be a valid range of numbers (i.e.
      1-3,6-9,11) or a specific section or chapter name as it appears in the
      config file.\n
  -v  Enable verbose output.\n";


pub fn run(args: &HashMap<String,String>) {

    match args.get(parse_args::HELP_ARG) {
        Some(_value) => {
            println!("{}", HELP_MESSAGE);
            return;
        },
        None => {}
    };

    let filename = match args.get(parse_args::FILENAME_ARG) {
        Some(string) => string,
        None => config::DEFAULT_CONFIG,
    };

    let outline = match config::read(filename) {
        Some(outline) => outline,
        None => return,
    };

    if let Some(_s) = args.get(parse_args::CLEAN_ARG) {
        compile::clean(args);
        return
    }

    if let Some(_s) = args.get(parse_args::VERBOSE_ARG) {
        println!("The outline I read was:\n\n{}\n", outline);
    }

    compile::compile(outline, &args);
}
