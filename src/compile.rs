use std::process::Command;
use std::collections::HashMap;

const DEFAULT_PREAMBLE: &str = "\\documentclass{article}
\\usepackage{amsmath,amstext,amssymb}";

enum Engine {
    LATEX,
    PDFLATEX,
}

struct ClassOption {
    class: String, 
    option: String,
}

struct CMD {
    engine: Engine,
    jobname: String,
    class_options: Vec<ClassOption>,
} 

impl CMD {
    pub fn new() -> CMD {
        CMD {
            engine: Engine::PDFLATEX,
            jobname: String::from("out"),
            class_options: Vec::<ClassOption>::new(),
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
        let mut tex = String::new();
        tex.push_str("\"");
        l
    }


}

pub fn compile(options: HashMap<&str, &str>) {
    let proposed_engine = options.get("engine").unwrap();
    let jobname = options.get("jobname").unwrap();
    let range: u8 = options.get("range").unwrap()
        .parse()
        .expect("Value of range should not be high.");


}


pub fn create_command(proposed_engine: &str) {
    let engine: Engine = match proposed_engine {
        "latex" => Engine::LATEX,
        "pdflatex" => Engine::PDFLATEX,
        _ => Engine::PDFLATEX,
    };

    let mut cmd = CMD::new();

    cmd.engine = engine;
    println!("{:?}", cmd.to_list());
}
