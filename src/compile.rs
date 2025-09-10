use std::process::Command;
use std::collections::HashMap;
use number_range::NumberRangeOptions;

use crate::config::Outline;

const DEFAULT_ENGINE: &str = "pdflatex";
const DEFAULT_RANGE: &str = "1";
const DEFAULT_JOBNAME: &str = "job";

enum Engine {
    LATEX,
    PDFLATEX,
}

struct CMD {
    engine: Engine,
    jobname: String,
} 

impl CMD {
    pub fn new() -> CMD {
        CMD {
            engine: Engine::PDFLATEX,
            jobname: String::from("out"),
        }

    }
    pub fn to_list(&self) -> Vec<&str> {
        match self.engine {
            Engine::PDFLATEX => self.pdflatex_list(),
            _ => Vec::<&str>::new(),
        }
    }

    fn pdflatex_list(&self) -> Vec<&str> {
        let mut l = Vec::<&str>::new();
        l.push("pdflatex");
        l.push("-jobname");
        l.push( match self.jobname.as_str() {
            "" => "out",
            _ => self.jobname.as_str(),
        });
        l
    }


}

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

    /* Debuging stuff, to be removed. */
    println!("proposed engine: {proposed_engine}");
    println!("jobname: {jobname}");
    println!("{rng:?}");
    /**********************************/

    let cmd = create_command(proposed_engine, jobname);


    println!("Printing out the command:");
    println!("{:?}", cmd.to_list());

    let mut tex = String::new();
    tex.push_str(outline.get_preamble().as_str());
    tex.push_str(outline.get_range(&rng).as_str());
    println!("{}", tex);


}


fn create_command(proposed_engine: &str, jobname: &str) -> CMD {
    let engine: Engine = match proposed_engine {
        "latex" => Engine::LATEX,
        "pdflatex" => Engine::PDFLATEX,
        _ => Engine::PDFLATEX,
    };

    let mut cmd = CMD::new();

    cmd.engine = engine;
    cmd.jobname = jobname.to_string();

    cmd
}

