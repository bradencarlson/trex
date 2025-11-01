/* cmd.rs
 *
 * Author: Braden Carlson
 * Date: September 2025
 *
 * Provides the CMD struct, which holds all the neccessary information to
 * compile the document with the appropriate command. 
 */

use std::process::Command;
use std::collections::HashMap;

use crate::outline::Outline;

use std::fmt;

pub enum Engine {
    LATEX,
    PDFLATEX,
}

pub struct CMD {
    pub engine: Engine,
    pub jobname: String,
    pub range: Vec<usize>,
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
            outline: Outline::new(),
            class_options: Vec::<String>::new(),
        }

    }
    pub fn run(&self) {
        /* Runs this command 
         *
         * Parameters: 
         *  options - a HashMap of additional options to use while 
         *            running this command. 
         */

        let mut command_list = self.command_list();
        match self.engine {
            Engine::PDFLATEX => {
                command_list.push( self.get_tex_code() );
            },
            _ => {}
        };
        let mut c = Command::new(&command_list[0]);
        c.args(&command_list[1..]);
        c.status().expect("Something went wrong while compiling the document.");
    }

    fn command_list(&self) -> Vec<String> {
        /*  Generates a list of parameters to be passed to a shell. 
         *
         *  Returns: 
         *    Vec<String> - a list of strings which will be passed to a shell when this command is
         *                  run.
         */

        let mut l = Vec::<String>::new();

        match self.engine {
            Engine::LATEX => {
                l.push("latex".to_string());
                let mut j = String::from("-jobname=");
                j.push_str( match self.jobname.as_str() {
                    "" => "out", 
                    _ => self.jobname.as_str(),
                });
                l.push(j);
            },

            /* Engine::PDFLATEX is the default, so it is not specified and 
             * is caught here. */
            _ => {
                l.push("pdflatex".to_string());
                let mut j = String::from("-jobname=");
                j.push_str( match self.jobname.as_str() {
                    "" => "out", 
                    _ => self.jobname.as_str(),
                });
                l.push(j);
            },
        };

        l
    }

    fn get_tex_code(&self) -> String {
        /* Gets the TeX code of the document, based on the specified range and the structure of the
         * outline file. 
         *
         * Returns: 
         *  String - a string of TeX code. 
         */

        let mut t = String::new();
        t.push_str("\"");

        for option in &self.class_options {
            t.push_str("\\PassOptionsToClass{");
            t.push_str(option.as_str());
            t.push_str("}{");
            t.push_str(self.outline.class.as_str());
            t.push_str("}");
        }

        t.push_str("\\input{");
        t.push_str(self.outline.preamble.as_str());
        t.push_str("}");

        t.push_str("\\begin{document}");

        t.push_str(self.get_document_content().as_str());
        
        t.push_str("\\end{document}");
        
        t.push_str("\"");
        t
    }

    fn get_document_content(&self) -> String {
        /* Used by get_tex_code to obtain the TeX code of the document based on the range which was
         * passed to this command. 
         *
         * Returns: 
         *  String - a string of TeX code
         */

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

        c.push_str("\\setcounter{section}{");
        c.push_str((section_idx-1).to_string().as_str());
        c.push_str("}");
        c.push_str("\\section{");
        c.push_str(self.outline.section_names[section_idx-1].as_str());
        c.push_str("}");
        
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
                            c.push_str("}");
                            section_idx += 1;
                        }
                    }
            }
            c.push_str("\\setcounter{subsection}{");
            c.push_str((file_idx - self.outline.section_positions[section_idx-1]).to_string().as_str());
            c.push_str("}");
            c.push_str("\\input{");
            c.push_str(self.outline.lecture_files[file_idx].as_str());
            c.push_str("}");
            file_idx += 1;
        }
        c
    }


}

impl fmt::Display for CMD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\n", self.command_list());
        write!(f, "The code appended to the command will be:\n");
        write!(f, "{}", self.get_tex_code())
    }
}

#[cfg(test)]
mod pdflatex {
    use super::*;

    fn create_outline() -> Outline {
        let mut o = Outline::new();
        o.class = String::from("article");
        o.preamble = String::from("preamble.tex");
        o.lecture_files = vec![String::from("file-1"), String::from("file-2"), 
            String::from("file-3"), String::from("file-4"),
            String::from("file-5"), String::from("file-6"), 
            String::from("file-7"), String::from("file-8"), 
            String::from("file-9"), String::from("file-10")];
        o.section_positions = vec![0,3,7];
        o.section_names = vec![
            String::from("Files 1-3"), 
            String::from("Files 4-7"), 
            String::from("Files 8-10")];
        o
    }

    #[test]
    fn lecture_six() {
        let o: Outline = create_outline();
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![6];
        c.jobname = String::from("lecture-6");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\\setcounter{section}{1}\
            \\section{Files 4-7}\\setcounter{subsection}{2}\\input{file-6}\\end{document}\"");
    }

    #[test]
    fn lectures_1_to_3() {
        let o: Outline = create_outline();
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3];
        c.jobname = String::from("lectures-1-3");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\\setcounter{section}{0}\
            \\section{Files 1-3}\\setcounter{subsection}{0}\\input{file-1}\
            \\setcounter{subsection}{1}\\input{file-2}\
            \\setcounter{subsection}{2}\\input{file-3}\
            \\end{document}\"");
    }

    #[test]
    fn lecture_10() {
        let o: Outline = create_outline();
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![10];
        c.jobname = String::from("lectures-1-3");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\\setcounter{section}{2}\
            \\section{Files 8-10}\\setcounter{subsection}{2}\\input{file-10}\\end{document}\"");
    }

    #[test]
    fn full_lecture() {
        let o: Outline = create_outline();
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7,8,9,10];
        c.jobname = String::from("lectures");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\\setcounter{section}{0}\
            \\section{Files 1-3}\\setcounter{subsection}{0}\\input{file-1}\
            \\setcounter{subsection}{1}\\input{file-2}\
            \\setcounter{subsection}{2}\\input{file-3}\
            \\section{Files 4-7}\\setcounter{subsection}{0}\\input{file-4}\
            \\setcounter{subsection}{1}\\input{file-5}\
            \\setcounter{subsection}{2}\\input{file-6}\
            \\setcounter{subsection}{3}\\input{file-7}\
            \\section{Files 8-10}\
            \\setcounter{subsection}{0}\\input{file-8}\
            \\setcounter{subsection}{1}\\input{file-9}\
            \\setcounter{subsection}{2}\\input{file-10}\
            \\end{document}\"");
    }

    #[test]
    fn jobname() {
        let o: Outline = create_outline();
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7,8,9,10];
        c.jobname = String::from("lectures");

        let l = c.command_list();
        
        assert_eq!(l[1], "-jobname=lectures");
    }

    #[test]
    fn engine() {
        let o: Outline = create_outline();
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7,8,9,10];
        c.jobname = String::from("lectures");

        let l = c.command_list();
        
        assert_eq!(l[0], "pdflatex");
    }

    #[test]
    fn nopres_option() {
        let o: Outline = create_outline();
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![6];
        c.jobname = String::from("lecture-6");
        c.class_options = vec![
            String::from("nopresentation")];

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\PassOptionsToClass{nopresentation}{article}\
            \\input{preamble.tex}\\begin{document}\\setcounter{section}{1}\
            \\section{Files 4-7}\
            \\setcounter{subsection}{2}\\input{file-6}\\end{document}\"");
    }

    #[test]
    fn two_options() {
        let o: Outline = create_outline();
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![6];
        c.jobname = String::from("lecture-6");
        c.class_options = vec![
            String::from("nopresentation"), 
            String::from("12pt")];

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\PassOptionsToClass{nopresentation}{article}\
            \\PassOptionsToClass{12pt}{article}\
            \\input{preamble.tex}\\begin{document}\\setcounter{section}{1}\
            \\section{Files 4-7}\\setcounter{subsection}{2}\\input{file-6}\\end{document}\"");
    }

}
