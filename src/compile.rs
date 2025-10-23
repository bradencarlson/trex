/* compile.rs
 *
 * Author: Braden Carlson
 * Date: September 2025
 *
 * Takes an outline and a HashMap of options to construct a command to run
 * and runs it.
 */

use std::collections::HashMap;
use number_range::NumberRangeOptions;

use crate::outline::Outline;
use crate::parse_args;

const DEFAULT_ENGINE: &str = "pdflatex";
const DEFAULT_RANGE: &str = "1";
const DEFAULT_JOBNAME: &str = "job";

mod cmd;

use cmd::CMD;
use cmd::Engine;


pub fn compile(outline: Outline, options: &HashMap<String, String>) {
    /* parses some of the options found in the options passed, then creates
     * the appropriate command and runs it. 
     *
     * Parameters: 
     *  outline - the outline of the lecture notes to use
     *  options - options to use when creating the command
     */

    let proposed_engine = match options.get(parse_args::ENGINE_ARG) {
        Some(string) => string.as_str(),
        None => DEFAULT_ENGINE,
    };
    let jobname = match options.get(parse_args::JOBNAME_ARG) {
        Some(string) => string.as_str(),
        None => DEFAULT_JOBNAME,
    };
    let range = match options.get(parse_args::RANGE_ARG) {
        Some(string) => string.as_str(),
        None => DEFAULT_RANGE,
    };

    let rng: Vec<usize> = match NumberRangeOptions::default()
        .with_range_sep('-')
        .parse(range) {
            Ok(vec) => vec.collect(),
            Err(_) => vec![1],
        };

    let mut class_options = Vec::<String>::new();

    for (key,value) in options {
        if value.as_str() == "class_option" {
            println!("option found: {}", key);
            class_options.push(key.to_string());
        }
    }

    let cmd = create_command(proposed_engine, jobname, rng, outline, class_options);

    if let Some(s) = options.get(parse_args::DRYRUN_ARG) {
        println!("Running the following command:");
        println!("{cmd}");
    } else {
        cmd.run();
    }
}


fn create_command(proposed_engine: &str, jobname: &str, 
    range: Vec<usize>, outline: Outline,class_options: Vec<String>) -> CMD {
    /* Creates a command object which can then be run to compile the document. 
     *
     * Parameters: 
     *  proposed_engine - the engine to use
     *  jobname         - the jobname to use
     *  range           - the range of files to compile
     *  outline         - the outline to use 
     *
     * Returns: 
     *  CMD             - the command which the caller can run
     */

    let engine: Engine = match proposed_engine {
        "latex" => Engine::LATEX,
        "pdflatex" => Engine::PDFLATEX,
        _ => Engine::PDFLATEX,
    };

    let mut cmd = CMD::new();

    cmd.engine = engine;
    cmd.jobname = jobname.to_string();
    cmd.range = range;
    cmd.outline = outline;
    cmd.class_options = class_options;

    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate(engine: &str) -> CMD {
        let jobname = "test_name";
        let range: Vec<usize> = vec![1];
        let outline = Outline::new();
        let class_options = Vec::<String>::new();

        create_command(engine, jobname, range, outline, class_options)
    }


    #[test]
    fn engine_pdflatex() {

        let c = generate("pdflatex");

        assert!( match c.engine {
            Engine::PDFLATEX => true, 
            _ => false
        });
    }

    #[test]
    fn engine_latex() {

        let c = generate("latex");

        assert!( match c.engine {
            Engine::LATEX => true, 
            _ => false
        });
    }

    
}
