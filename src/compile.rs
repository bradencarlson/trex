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
use std::fs;

use crate::outline::Outline;
use crate::parse_args;

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
        /* Runs this command */

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

                /* Notice that if no chapter commands are found in the config file.
                 * Then each of the chapter indices will be zero, so this block will
                 * never be run, as desired. The same happens for sections below. */
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

        if self.outline.bib_style.len() > 0 {
            c.push_str("\\bibliographystyle{");
            c.push_str(self.outline.bib_style.as_str());
            c.push_str("}");

            c.push_str("\\bibliography{");
            c.push_str(self.outline.bib_file.as_str());
            c.push_str("}");
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

pub fn clean(options: &HashMap<String,String>) {
    /* Removes auxiliary files created during the compilation process. */
    let pdflatex = String::from("pdflatex");
    match options.get(parse_args::ENGINE_ARG) {
       Some(pdflatex) => {
            let mut files = Vec::<String>::new();
            let file_types = vec![".aux", ".log", ".toc", ".bbl", ".blg"];
            if let Ok(dir) = fs::read_dir(".") {
                for entry in dir {
                    if let Ok(e) = entry {
                        let e_is_file = e.file_type()
                            .expect("Something went wrong reading directory")
                            .is_file();
                        if e_is_file {
                            let name = e.file_name();
                            let filename = name.to_str()
                                .expect("Something went wrong.");
                            let mut valid = false;
                            for ending in file_types.iter() {
                                if filename.ends_with(ending) {
                                    files.push(filename.to_string());
                                }
                            }
                        }
                    }

                }
            }

            println!("Files to be removed: {:?}", files);
            let mut c = Command::new(String::from("rm"));
            c.arg("-I");
            c.args(files);
            c.status().expect("Something went wrong during cleaning");

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
            "examples/default.conf") {
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

    #[test]
    fn lectures_1_to_3_with_subsections() {
        let o: Outline = match config::read(
            "examples/notes.conf") {
            Some(outline) => outline,
            None => Outline::new(),
        };
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3];
        c.jobname = String::from("lectures-1-3");

        assert_eq!(c.outline.chapter_indices, vec![0,0,0,0,0,0,0]);

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\\setcounter{section}{0}\
            \\section{Voting Theory}\
            \\setcounter{subsection}{0}\
            \\input{lecture-1.tex}\
            \\setcounter{subsection}{1}\
            \\input{lecture-2.tex}\
            \\setcounter{subsection}{2}\
            \\input{lecture-3.tex}\
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

    #[test]
    fn full_lecture_with_subsections() {
        let o: Outline = match config::read(
            "examples/notes.conf") {
            Some(outline) => outline,
            None => Outline::new(),
        };
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7];
        c.jobname = String::from("lectures");

        let tex = c.get_tex_code();
        assert_eq!(tex, "\"\\input{preamble.tex}\\begin{document}\
            \\setcounter{section}{0}\
            \\section{Voting Theory}\
            \\setcounter{subsection}{0}\
            \\input{lecture-1.tex}\
            \\setcounter{subsection}{1}\
            \\input{lecture-2.tex}\
            \\setcounter{subsection}{2}\
            \\input{lecture-3.tex}\
            \\setcounter{subsection}{3}\
            \\input{lecture-4.tex}\
            \\setcounter{subsection}{4}\
            \\input{lecture-5.tex}\
            \\setcounter{section}{1}\
            \\section{Weighted Voting Theory}\
            \\setcounter{subsection}{0}\
            \\input{lecture-6.tex}\
            \\setcounter{subsection}{1}\
            \\input{lecture-7.tex}\
            \\end{document}\"");
    }

    #[test]
    fn handle_sections_false() {
        let o: Outline = match config::read(
            "examples/book.conf") {
            Some(outline) => outline,
            None => Outline::new(),
        };
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![2,3];
        c.jobname = String::from("out");

        let tex = c.get_tex_code();

        assert_eq!(tex,"\"\\input{preamble.tex}\
            \\begin{document}\
            \\setcounter{section}{0}\
            \\input{chapter-1/chapter-1-part-2.tex}\
            \\input{chapter-2/chapter-2.tex}\
            \\end{document}\"");

    }

    #[test]
    fn section_name_as_range() {
        let o: Outline = match config::read(
            "examples/default.conf" ) {
            Some(outline) => outline,
            None => Outline::new(),
        };

        let mut c = CMD::new();
        c.outline = o;
        c.range = generate_range_from_name("Files 4-7", &c.outline);

        let t = c.get_tex_code();

        assert_eq!(t, "\"\
            \\input{preamble.tex}\
            \\begin{document}\
            \\setcounter{chapter}{0}\
            \\chapter{Chapter 1}\
            \\setcounter{section}{1}\
            \\section{Files 4-7}\
            \\input{file-4}\
            \\input{file-5}\
            \\input{file-6}\
            \\input{file-7}\
            \\end{document}\"");
    }

    #[test]
    fn chapter_name_as_range() {
        let o: Outline = match config::read(
            "examples/default.conf" ) {
            Some(outline) => outline,
            None => Outline::new(),
        };

        let mut c = CMD::new();
        c.outline = o;
        c.range = generate_range_from_name("Chapter 2", &c.outline);

        let t = c.get_tex_code();

        assert_eq!(t, "\"\
            \\input{preamble.tex}\
            \\begin{document}\
            \\setcounter{chapter}{1}\
            \\chapter{Chapter 2}\
            \\setcounter{section}{0}\
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

    #[test]
    fn bibliography() {
        let o: Outline = match config::read(
            "examples/default-with-bib.conf") {
            Some(outline) => outline,
            None => Outline::new()
        };
        let mut c = CMD::new();
        c.outline = o;
        c.range = vec![4];
        c.jobname = String::from("out");

        let tex = c.get_tex_code();

        assert_eq!(tex, "\"\\input{preamble.tex}\
            \\begin{document}\
            \\setcounter{chapter}{0}\
            \\chapter{Chapter 1}\
            \\setcounter{section}{1}\
            \\section{Files 4-7}\
            \\input{file-4}\
            \\bibliographystyle{plain}\
            \\bibliography{sources.bib}\
            \\end{document}\"");

    }


    #[test]
    fn section_num_arg() {
        let o: Outline = match config::read(
            "examples/notes.conf") {
            Some(outline) => outline, 
            None => Outline::new()
        };

        let arg = "s1";
        let v = generate_range_from_name(arg, &o);

        let arg2 = "s2";
        let v2 = generate_range_from_name(arg2, &o);
        
        assert_eq!(v, vec![1,2,3,4,5]);
        assert_eq!(v2, vec![6,7]);
        
    }

    #[test]
    fn chapter_num_arg() {
        let o: Outline = match config::read(
            "examples/default.conf") {
            Some(outline) => outline, 
            None => Outline::new()
        };

        let arg = "c1";
        let v = generate_range_from_name(arg, &o);

        assert_eq!(v, vec![1,2,3,4,5,6,7]);
    }

    #[test]
    fn chapter_and_section_num() {
        let o: Outline = match config::read(
            "examples/default.conf" ) {
            Some(outline) => outline, 
            None => Outline::new()
        };

        let arg = "c1s2";
        let v = generate_range_from_name(arg, &o);

        assert_eq!(v, vec![4,5,6,7]);
    }

    #[test]
    fn select_all() {
        let o: Outline = match config::read(
            "examples/default.conf") {
            Some(outline) => outline, 
            None => Outline::new()
        };

        let arg = "all";
        let v = generate_range_from_name(arg, &o);

        assert_eq!(v, vec![1,2,3,4,5,6,7,8,9,10]);
    }

}
