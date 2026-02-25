/* compile.rs
 *
 * Author: Braden Carlson
 * Date: September 2025
 *
 * Takes an outline and a HashMap of options to construct a command to run
 * and runs it.
 */

use std::collections::HashMap;
use std::str::FromStr;
use number_range::NumberRangeOptions;
use std::process::exit;
use std::process::Command;
use std::fmt;

use crate::config::Outline;
use crate::parse_args;

mod tex;

const DEFAULT_ENGINE: &str = "pdflatex";
const DEFAULT_RANGE: &str = "1";
const DEFAULT_JOBNAME: &str = "job";

pub enum Engine {
    LATEX,
    PDFLATEX,
}

pub struct CMD {
    pub engine: Engine,
    pub jobname: String,
    pub range: Vec<usize>,
    pub quiet: bool,
    pub outline: Outline,
    pub class_options: Vec<String>,
}

impl CMD {
    pub fn new() -> CMD {
        /* Creates a new CMD, with some good default values. */

        CMD {
            engine: Engine::PDFLATEX,
            jobname: String::from("out"),
            range: Vec::<usize>::new(),
            quiet: false,
            outline: Outline::new(),
            class_options: Vec::<String>::new(),
        }

    }

    pub fn run(&self) {
        /* Runs this command */

        match self.engine {
            Engine::PDFLATEX => {
                tex::run(&self);
            },
            _ => {}
        };
    }

    pub fn get_code(&self) -> String {
        match self.engine {
            Engine::PDFLATEX => {
                let mut code = tex::get_code(&self.range, &self.outline, &self.class_options);
                // the tex submodule wraps the code in "'s, so remove these before returning.
                code.retain(|c| c != '\"');
                return code;
            },
            _ => {
                return String::new();
            }
        };

    }

}

impl fmt::Display for CMD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The code appended to the command will be:\n");
        match self.engine {
            Engine::PDFLATEX => {
                write!(f, "{}\n", tex::get_code(&self.range, &self.outline, &self.class_options));
            }
            _ => {
                write!(f, "Invalid engine detected.\n");
            }
        }
        write!(f, "Quiet mode enabled: {}", self.quiet)

    }
}

pub fn clean(options: &HashMap<String,String>) {
    /* Removes auxiliary files created during the compilation process. */
    let pdflatex = String::from("pdflatex");
    match options.get(parse_args::ENGINE_ARG) {
        Some(pdflatex) => {
            tex::clean();
        }
        None => {
           println!("Nothing to do!");
        }
    }
}

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
        None => {
            println!("No range given.");
            exit(1);
        },
    };

    let rng: Vec<usize> = match NumberRangeOptions::default()
        .with_range_sep('-')
        .parse(range) {
            Ok(vec) => vec.collect(),
            Err(_) => {
                let v: Vec<usize> = generate_range_from_name(range, &outline);
                v
                //exit(2);
            },
        };

    let mut class_options = Vec::<String>::new();

    for (key,value) in options {
        if value.as_str() == "class_option" {
            println!("option found: {}", key);
            class_options.push(key.to_string());
        }
    }

    let mut cmd = create_command(proposed_engine, jobname, rng, outline, class_options);

    if let Some(_s) = options.get(parse_args::QUIET_ARG) {
        cmd.quiet = true;
    }

    if let Some(_s) = options.get(parse_args::DRYRUN_ARG) {
        println!("{cmd}");
    } else if let Some(_s) = options.get(parse_args::CODE_ARG) {
        println!("{}",cmd.get_code());
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

fn generate_range_from_name(name: &str, outline: &Outline) -> Vec<usize> {
    /* Generates a valid range from a given section or chapter name.
     *
     * Parameters:
     *   name    - the section/chapter name.
     *   outline - the outline which has been read.
     *
     * Returns:
     *   A vector containing the relevant file numbers to be included in
     *   the output.
     */
    let mut v: Vec<usize> = Vec::<usize>::new();

    let mut idx: usize = 1;
    let mut processed = false;
    let num_files = outline.files.len();

    for sec in outline.section_names.iter() {
        /* Currently, it seems that while parsing the configuration
         * file, the resulting Outline contains one too many section
         * and chapter names. While this does not affect compilation if
         * a regular range is given, using iterators here picks up those
         * extra values, and throws an error later. This will need to be
         * fixed in the config module. For now, do a simple check to
         * make sure that only valid file numbers are spit out. */
        if idx <= num_files && sec == name {
            processed = true;
            v.push(idx);
        }
        idx += 1;
    }

    // reset idx to 1
    idx = 1;

    if !processed {
        for chap in outline.chapter_names.iter() {
            if idx <= num_files && chap == name {
                processed = true;
                v.push(idx);
            }
            idx += 1;
        }
    }

    idx = 1;

    if !processed {
        /* If the given arg starts with s, and was not a section or chapter
         * name, then we assume that there is no chapters in the document
         * (otherwise they would have specified this as the argument), so we
         * pick up all files whose corresponding section number matches the
         * number given.  */
        if name.starts_with("s") {
            let num = &name[1..];
            let num_actual: i32 = i32::from_str(num).unwrap_or(1);
            for sec in outline.section_indices.iter() {
                if *sec == num_actual {
                    processed = true;
                    v.push(idx);
                }
                idx += 1;
            }
        }

        if name.starts_with("c") {
            let nums: Vec<&str> = name[1..].split(char::is_alphabetic).collect();
            let num_one: i32 = match nums.get(0) {
                Some(num) => i32::from_str(num).unwrap_or(1),
                None => 1
            };
            let num_two: i32 = match nums.get(1) {
                Some(num) => i32::from_str(num).unwrap_or(-1),
                None => -1
            };
            if num_two == -1 {
                /* The user wants an entire chapter, no need to check section
                 * indicies here. */
                for chap in outline.chapter_indices.iter() {
                    if *chap == num_one {
                        processed = true;
                        v.push(idx);
                    }
                    idx += 1;
                }
            } else {
                /* The user wants a specific section of a given chapter */
                idx = 0;
                while idx < num_files {
                    let chap = &outline.chapter_indices;
                    let sec = &outline.section_indices;

                    let chap_idx = chap.get(idx).unwrap_or(&-1);
                    let sec_idx = sec.get(idx).unwrap_or(&-1);

                    if *chap_idx == num_one && *sec_idx == num_two {
                        processed = true;
                        v.push(idx + 1);
                    }

                    idx += 1;
                }
            }
        }

    }

    idx = 1;

    /* Finally, if nothing else worked, check to see if the argument was the
     * 'all' keyword */
    if !processed {
        if name == "all" {
            for file in outline.files.iter() {
                v.push(idx);
                idx += 1;
            }
        }
    }


    return v;
}

#[cfg(test)]
mod command_tests {
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

