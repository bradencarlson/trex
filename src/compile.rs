use std::process::Command;
use std::collections::HashMap;
use number_range::NumberRangeOptions;

use crate::outline::Outline;

const DEFAULT_ENGINE: &str = "pdflatex";
const DEFAULT_RANGE: &str = "1";
const DEFAULT_JOBNAME: &str = "job";

mod cmd;

use cmd::CMD;
use cmd::Engine;


pub fn compile(outline: &Outline, options: &HashMap<String, String>) {
    let proposed_engine = match options.get("engine") {
        Some(string) => string.as_str(),
        None => DEFAULT_ENGINE,
    };
    let jobname = match options.get("jobname") {
        Some(string) => string.as_str(),
        None => DEFAULT_JOBNAME,
    };
    let range = match options.get("range") {
        Some(string) => string.as_str(),
        None => DEFAULT_RANGE,
    };

    let rng: Vec<usize> = match NumberRangeOptions::default()
        .with_range_sep('-')
        .parse(range) {
            Ok(vec) => vec.collect(),
            Err(_) => vec![1],
        };

    let cmd = create_command(proposed_engine, jobname, rng);
}


fn create_command(proposed_engine: &str, jobname: &str, range: Vec<usize>) -> CMD {
    let engine: Engine = match proposed_engine {
        "latex" => Engine::LATEX,
        "pdflatex" => Engine::PDFLATEX,
        _ => Engine::PDFLATEX,
    };

    let mut cmd = CMD::new();

    cmd.engine = engine;
    cmd.jobname = jobname.to_string();
    cmd.range = range;

    cmd
}

