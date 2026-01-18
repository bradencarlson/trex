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
        let num_files = self.outline.files.len();
        let num_sections = self.outline.section_positions.len();
        let num_chapters = self.outline.chapter_positions.len();
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
        let mut last_chapter_idx = -1;
        let mut last_section_idx = -1;


        loop {
            if file_idx >= end {
                break;
            }

            if self.range.contains(&(file_idx+1)) {
                let chap = self.outline.chapter_indices[file_idx]-1;
                let sec = self.outline.section_indices[file_idx]-1;

                if chap != last_chapter_idx {
                    c.push_str("\\setcounter{chapter}{");
                    c.push_str(chap.to_string().as_str());
                    c.push_str("}");
                    last_chapter_idx = chap;

                    if self.outline.handle_chapters {
                        let chap_name = self.outline.chapter_names[file_idx].as_str();
                        c.push_str("\\chapter{");
                        c.push_str(chap_name);
                        c.push_str("}");
                    }
                }

                if sec != last_section_idx {
                    c.push_str("\\setcounter{section}{");
                    c.push_str(sec.to_string().as_str());
                    c.push_str("}");
                    last_section_idx = sec;

                    if self.outline.handle_sections {
                        let sec_name = self.outline.section_names[file_idx].as_str();
                        c.push_str("\\section{");
                        c.push_str(sec_name);
                        c.push_str("}");
                    }
                }

                if self.outline.handle_subsections {
                    let subsec_num = self.outline.subsection_indices[file_idx].to_string();
                    c.push_str("\\setcounter{subsection}{");
                    c.push_str(subsec_num.as_str());
                    c.push_str("}");
                }

                c.push_str("\\input{");
                c.push_str(self.outline.files[file_idx].as_str());
                c.push_str("}");
            }

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
    use crate::outline;
    use crate::config;

    fn create_outline(subsections: bool) -> Outline {
        let mut o = Outline::new();
        o.class = String::from("article");
        o.preamble = String::from("preamble.tex");
        o.files = vec![String::from("file-1"), String::from("file-2"), 
            String::from("file-3"), String::from("file-4"),
            String::from("file-5"), String::from("file-6"), 
            String::from("file-7"), String::from("file-8"), 
            String::from("file-9"), String::from("file-10")];
        o.handle_subsections = subsections;
        o.section_positions = vec![1,0,0,1,0,0,0,1,0,0];
        o.section_names = vec![
            String::from("Files 1-3"), 
            String::from("Files 1-3"), String::from("Files 1-3"),
            String::from("Files 4-7"), 
            String::from("Files 4-7"), String::from("Files 4-7"), String::from("Files 4-7"),
            String::from("Files 8-10"), String::from("Files 8-10"), String::from("Files 8-10")];
        o.section_indices = vec![1,1,1,2,2,2,2,1,1,1];
        o.chapter_positions = vec![1,0,0,0,0,0,0,1,0,0];
        o.chapter_names = vec![
            String::from("Chapter 1"),
            String::from("Chapter 1"),String::from("Chapter 1"),
            String::from("Chapter 1"),String::from("Chapter 1"),
            String::from("Chapter 1"),String::from("Chapter 1"),
            String::from("Chapter 2"),
            String::new(),String::new()];
        o.chapter_indices = vec![1,1,1,1,1,1,1,2,2,2];

        o
    }

    #[test]
    fn lecture_six() {
        let o: Outline = match config::read(
            "/home/bradencarlson/Documents/trex/examples/default.conf") {
            Some(outline) => outline,
            None => Outline::new(),
        };
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![6];
        c.jobname = String::from("lecture-6");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\
            \\setcounter{chapter}{0}\
            \\chapter{Chapter 1}\
            \\setcounter{section}{1}\
            \\section{Files 4-7}\\input{file-6}\\end{document}\"");
    }

    #[test]
    fn lectures_1_to_3() {
        let o: Outline = match config::read(
            "examples/default.conf") {
            Some(outline) => outline, 
            None => Outline::new(),
        };

        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3];
        c.jobname = String::from("lectures-1-3");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\
            \\setcounter{chapter}{0}\\chapter{Chapter 1}\
            \\setcounter{section}{0}\
            \\section{Files 1-3}\\input{file-1}\
            \\input{file-2}\
            \\input{file-3}\
            \\end{document}\"");
    }

    /* #[test]
     * Since I do not currently have subsection numbering handled, ignore this test 
     * for now. */
    fn lectures_1_to_3_nosubs() {
        let o: Outline = create_outline(false);
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3];
        c.jobname = String::from("lectures-1-3");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\\setcounter{section}{0}\
            \\section{Files 1-3}\\input{file-1}\
            \\input{file-2}\
            \\input{file-3}\
            \\end{document}\"");
    }

    #[test]
    fn lecture_10() {
        let o: Outline = match config::read(
            "examples/default.conf") {
            Some(outline) => outline, 
            None => Outline::new(),
        };

        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![10];
        c.jobname = String::from("lectures-1-3");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\
            \\setcounter{chapter}{1}\
            \\chapter{Chapter 2}\
            \\setcounter{section}{0}\
            \\section{Files 8-10}\
            \\input{file-10}\\end{document}\"");
    }


    #[test]
    fn full_lecture() {
        let o: Outline = match config::read(
            "examples/default.conf") {
            Some(outline) => outline,
            None => Outline::new(),
        };
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7,8,9,10];
        c.jobname = String::from("lectures");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\
            \\setcounter{chapter}{0}\
            \\chapter{Chapter 1}\
            \\setcounter{section}{0}\
            \\section{Files 1-3}\\input{file-1}\
            \\input{file-2}\
            \\input{file-3}\
            \\setcounter{section}{1}\
            \\section{Files 4-7}\\input{file-4}\
            \\input{file-5}\
            \\input{file-6}\
            \\input{file-7}\
            \\setcounter{chapter}{1}\
            \\chapter{Chapter 2}\
            \\setcounter{section}{0}\
            \\section{Files 8-10}\
            \\input{file-8}\
            \\input{file-9}\
            \\input{file-10}\
            \\end{document}\"");
    }

    /* #[test]
     * Remove this test for now. */
    fn full_lecture_nosubs() {
        let o: Outline = create_outline(false);
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7,8,9,10];
        c.jobname = String::from("lectures");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\\setcounter{section}{0}\
            \\section{Files 1-3}\\input{file-1}\
            \\input{file-2}\
            \\input{file-3}\
            \\section{Files 4-7}\\input{file-4}\
            \\input{file-5}\
            \\input{file-6}\
            \\input{file-7}\
            \\section{Files 8-10}\
            \\input{file-8}\
            \\input{file-9}\
            \\input{file-10}\
            \\end{document}\"");
    }

    #[test]
    fn jobname() {
        let o: Outline = create_outline(true);
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7,8,9,10];
        c.jobname = String::from("lectures");

        let l = c.command_list();
        
        assert_eq!(l[1], "-jobname=lectures");
    }

    #[test]
    fn engine() {
        let o: Outline = create_outline(true);
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7,8,9,10];
        c.jobname = String::from("lectures");

        let l = c.command_list();
        
        assert_eq!(l[0], "pdflatex");
    }

    #[test]
    fn nopres_option() {
        let o: Outline = match config::read(
            "examples/default.conf") {
            Some(outline) => outline, 
            None => Outline::new(),
        };
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![6];
        c.jobname = String::from("lecture-6");
        c.class_options = vec![
            String::from("nopresentation")];

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\PassOptionsToClass{nopresentation}{article}\
            \\input{preamble.tex}\\begin{document}\
            \\setcounter{chapter}{0}\
            \\chapter{Chapter 1}\
            \\setcounter{section}{1}\
            \\section{Files 4-7}\
            \\input{file-6}\\end{document}\"");
    }

    #[test]
    fn two_options() {
        let o: Outline = match config::read(
            "examples/default.conf") {
            Some(outline) => outline, 
            None => Outline::new(),
        };
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
            \\input{preamble.tex}\\begin{document}\
            \\setcounter{chapter}{0}\
            \\chapter{Chapter 1}\
            \\setcounter{section}{1}\
            \\section{Files 4-7}\
            \\input{file-6}\\end{document}\"");
    }

}
