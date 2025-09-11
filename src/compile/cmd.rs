use std::process::Command;

use crate::outline::Outline;

pub enum Engine {
    LATEX,
    PDFLATEX,
}

pub struct CMD {
    pub engine: Engine,
    pub jobname: String,
    pub range: Vec<usize>,
    pub outline: Outline,
} 

impl CMD {
    pub fn new() -> CMD {
        CMD {
            engine: Engine::PDFLATEX,
            jobname: String::from("out"),
            range: Vec::<usize>::new(),
            outline: Outline::new(),
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
