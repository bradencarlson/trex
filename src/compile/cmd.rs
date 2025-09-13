use std::process::Command;
use std::collections::HashMap;

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
    pub fn run(&self, options: &HashMap<String, String>) {
        println!("range is: {:?}", self.range);
        println!("{}", self.get_tex_code(options));
    }

    fn pdflatex_list(&self) -> Vec<String> {
        let mut l = Vec::<String>::new();
        l.push("pdflatex".to_string());
        let mut j = String::from("-jobname=");
        j.push_str( match self.jobname.as_str() {
            "" => "out", 
            _ => self.jobname.as_str(),
        });
        l.push(j);
        l
    }

    fn get_tex_code(&self, options: &HashMap<String, String>) -> String {
        let mut t = String::new();
        t.push_str("\"");

        t.push_str("\\input{");
        t.push_str(self.outline.preamble.as_str());
        t.push_str("}\n");

        t.push_str("\\begin{document}\n");

        t.push_str(self.get_document_content().as_str());
        
        t.push_str("\\end{document}");
        
        t.push_str("\"");
        t
    }

    fn get_document_content(&self) -> String {
        // TODO: Go though this entire method and check for things that could go wrong. This cannot
        // fail!
 
        let mut c = String::new();
        let num_files = self.outline.lecture_files.len();
        let num_sections = self.outline.section_positions.len();
        let start = self.range[0]-1;
        let end = self.range[self.range.len()-1];

        let mut max: usize = 0;

        /* Determine the maximum section position, this will be used to
         * make sure that the section positions do not exceed the number of
         * files which are to be input. 
         */
        for num in &self.outline.section_positions {
            if *num > max {
                max = *num;
            } 
        }
        if max >= num_files {
            println!("There is something wrong with the outline, the 
                position of the last section is greater than the number of files present.");
            return c
        }

        let mut file_idx = start;
        let mut section_idx = 0;

        /* Figure out which section we need to start with */
        loop {
            if section_idx < num_sections &&
                self.outline.section_positions[section_idx] <= start {
                    section_idx += 1;
            } else {
                break;
            }
        }

        c.push_str("\\setcounter{");
        c.push_str((section_idx-1).to_string().as_str());
        c.push_str("}\n");
        c.push_str("\\section{");
        c.push_str(self.outline.section_names[section_idx-1].as_str());
        c.push_str("}\n");
        
        loop {
            if file_idx >= end {
                break;
            }

            if section_idx < num_sections && 
                file_idx == self.outline.section_positions[section_idx] {
                    match self.engine {
                        // For now, always use the pdflatex code, eventually, 
                        // this will be changed based on what engine is selected. 
                        _ => {
                            c.push_str("\\section{");
                            c.push_str(self.outline.section_names[section_idx].as_str());
                            c.push_str("}\n");
                            section_idx += 1;
                        }
                    }
            }
            c.push_str("\\input{");
            c.push_str(self.outline.lecture_files[file_idx].as_str());
            c.push_str("}\n");
            file_idx += 1;
        }
        c
    }


}
