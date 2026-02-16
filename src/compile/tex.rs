/* tex.rs
 *
 * Author: Braden Carlson
 * Date: February 2026
 *
 * Implements the neccesary logic for the compilation of tex documents using
 * pdflatex. There are a few public functions: 
 *   clean        - clean up auxiliary files
 *   run          - compile the document
 *   get_tex_code - get the actual tex code which will be used.
 */ 

use std::process::Command;
use std::fs;
use std::io;
use std::hash::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::config::Outline;
use crate::utils::stack::TwoStack;
use super::CMD;

pub fn clean() {
    /* Cleans up auxiliary files created when compiling a tex document. */

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

pub fn run(cmd: &CMD) {
    /* Compiles the document using the parameters in the CMD struct passed to
     * the function. This will take care of multiple passes, running bibtex
     * if needed, etc.
     *
     * Paramters: 
     *   cmd - the CMD struct from which to read the outline, jobname, etc.
     */

    let mut aux: String = cmd.jobname.clone();
    let mut bbl: String = cmd.jobname.clone();
    aux.push_str(".aux");
    bbl.push_str(".bbl");

    // Define a stack with maximum height 2 to hold the hashes for the aux 
    // and bib files.
    let mut aux_hashes: TwoStack<u64> = TwoStack::new();
    let mut bib_hashes: TwoStack<u64> = TwoStack::new();

    match get_file_hash(&aux) {
        Some(h) => aux_hashes.push(h),
        None => {}
    };
    match get_file_hash(&bbl) {
        Some(h) => bib_hashes.push(h),
        None => {}
    };
    run_pdflatex(cmd);

    match get_file_hash(&aux) {
        Some(h) => aux_hashes.push(h),
        None => {}
    };
    println!("Current number of aux hashes: {}", aux_hashes.len);
}

fn run_pdflatex(cmd: &CMD) {
    /* Runs pdflatex with the appropriate tex code. 
     *
     * Paramters: 
     *   cmd - the CMD struct from which to read the outline, jobname, etc.
     */

    let cl = get_pdflatex_command_list(
        &cmd.jobname, 
        &cmd.range,
        &cmd.outline,
        &cmd.class_options
    );

    let mut c = Command::new(&cl[0]);
    c.args(&cl[1..]);
    c.status().expect("Something went wrong compiling the document.");
    
}

fn get_pdflatex_command_list(jobname: &String, range: &Vec<usize>,
    outline: &Outline, class_options: &Vec<String>) -> Vec<String> {
    /* Generates the actual list of strings which should make up the command
     * that is to be run when running pdflatex.
     *
     * Parameters: 
     *   jobname       - the name (without extension) of the output file.
     *   range         - the range of files to include.
     *   outline       - the outline of the document.
     *   class_options - any options which are to be passed to the class.
     */

    let mut cl = Vec::<String>::new();

    cl.push("pdflatex".to_string());
    let mut j = String::from("-jobname=");
    if jobname == "" {
        j.push_str("out");
    } else {
        j.push_str(jobname.as_str());
    }
    cl.push(j);
    cl.push(get_tex_code(range, outline, class_options));

    cl
}

pub fn get_tex_code(range: &Vec<usize>, outline: &Outline, class_options: &Vec<String>) -> String {
    /* Gets the TeX code of the document, based on the specified range and the structure of the
     * outline file.
     *
     * Returns:
     *  String - a string of TeX code.
     */

    let mut t = String::new();
    t.push_str("\"");

    for option in class_options {
        t.push_str("\\PassOptionsToClass{");
        t.push_str(option.as_str());
        t.push_str("}{");
        t.push_str(outline.class.as_str());
        t.push_str("}");
    }

    t.push_str("\\input{");
    t.push_str(outline.preamble.as_str());
    t.push_str("}");

    t.push_str("\\begin{document}");

    t.push_str(get_document_content(range, outline).as_str());

    t.push_str("\\end{document}");

    t.push_str("\"");
    t
}

fn get_document_content(range: &Vec<usize>, outline: &Outline) -> String {
    /* Used by get_tex_code to obtain the TeX code of the document based on the range which was
     * passed to this command.
     *
     * Returns:
     *  String - a string of TeX code
     */

    let mut c = String::new();
    let num_files = outline.files.len();
    let num_sections = outline.section_positions.len();
    let num_chapters = outline.chapter_positions.len();
    let start = range[0]-1;
    let end = range[range.len()-1];

    let mut max: usize = 0;

    /* Determine the maximum section position, this will be used to
     * make sure that the section positions do not exceed the number of
     * files which are to be input.
     */
    for num in &outline.section_positions {
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

        if range.contains(&(file_idx+1)) {
            let chap = outline.chapter_indices[file_idx]-1;
            let sec = outline.section_indices[file_idx]-1;

            /* Notice that if no chapter commands are found in the config file.
             * Then each of the chapter indices will be zero, so this block will
             * never be run, as desired. The same happens for sections below. */
            if chap != last_chapter_idx {
                c.push_str("\\setcounter{chapter}{");
                c.push_str(chap.to_string().as_str());
                c.push_str("}");
                last_chapter_idx = chap;

                if outline.handle_chapters {
                    let chap_name = outline.chapter_names[file_idx].as_str();
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

                if outline.handle_sections {
                    let sec_name = outline.section_names[file_idx].as_str();
                    c.push_str("\\section{");
                    c.push_str(sec_name);
                    c.push_str("}");
                }
            }

            if outline.handle_subsections {
                let subsec_num = outline.subsection_indices[file_idx].to_string();
                c.push_str("\\setcounter{subsection}{");
                c.push_str(subsec_num.as_str());
                c.push_str("}");
            }

            c.push_str("\\input{");
            c.push_str(outline.files[file_idx].as_str());
            c.push_str("}");
        }

        file_idx += 1;

    }

    if outline.bib_style.len() > 0 {
        c.push_str("\\bibliographystyle{");
        c.push_str(outline.bib_style.as_str());
        c.push_str("}");

        c.push_str("\\bibliography{");
        c.push_str(outline.bib_file.as_str());
        c.push_str("}");
    }

    c
}

fn get_file_hash(filename: &str) -> Option<u64> {
    let mut t = DefaultHasher::new();
    let file = match fs::read(filename) {
        Ok(f) => f, 
        Err(_) => return None
    };
    file.hash(& mut t);
    Some(t.finish())
}

#[cfg(test)]
mod pdflatex {
    use super::*;
    use crate::config;
    use crate::config::Outline;
    use crate::compile;

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
        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![6];
        c.jobname = String::from("lecture-6");

        let tex = get_tex_code(&c.range, &c.outline, &c.class_options);
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

        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![1,2,3];
        c.jobname = String::from("lectures-1-3");

        let tex = get_tex_code(&c.range, &c.outline, &c.class_options);
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
        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![1,2,3];
        c.jobname = String::from("lectures-1-3");

        assert_eq!(c.outline.chapter_indices, vec![0,0,0,0,0,0,0]);

        let tex = get_tex_code(&c.range, &c.outline, &c.class_options);
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

        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![10];
        c.jobname = String::from("lectures-1-3");

        let tex = get_tex_code(&c.range, &c.outline, &c.class_options);
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
        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7,8,9,10];
        c.jobname = String::from("lectures");

        let tex = get_tex_code(&c.range, &c.outline, &c.class_options);
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
        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7];
        c.jobname = String::from("lectures");

        let tex = get_tex_code(&c.range, &c.outline, &c.class_options);
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
        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![2,3];
        c.jobname = String::from("out");

        let tex = get_tex_code(&c.range, &c.outline, &c.class_options);

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

        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = compile::generate_range_from_name("Files 4-7", &c.outline);

        let t = get_tex_code(&c.range, &c.outline, &c.class_options);

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

        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = compile::generate_range_from_name("Chapter 2", &c.outline);

        let t = get_tex_code(&c.range, &c.outline, &c.class_options);

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
        let o: Outline = match config::read(
            "examples/default.conf") {
            Some(outline) => outline,
            None => Outline::new(),
        };
        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7,8,9,10];
        c.jobname = String::from("lectures");

        let l = get_pdflatex_command_list(
            &c.jobname,
            &c.range,
            &c.outline,
            &c.class_options);

        assert_eq!(l[1], "-jobname=lectures");
    }

    #[test]
    fn engine() {
        let o: Outline = match config::read(
            "examples/default.conf") {
            Some(outline) => outline,
            None => Outline::new()
        };
        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![1,2,3,4,5,6,7,8,9,10];
        c.jobname = String::from("lectures");

        let l = get_pdflatex_command_list(
            &c.jobname,
            &c.range,
            &c.outline,
            &c.class_options);

        assert_eq!(l[0], "pdflatex");
    }

    #[test]
    fn nopres_option() {
        let o: Outline = match config::read(
            "examples/default.conf") {
            Some(outline) => outline,
            None => Outline::new(),
        };
        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![6];
        c.jobname = String::from("lecture-6");
        c.class_options = vec![
            String::from("nopresentation")];

        let tex = get_tex_code(&c.range, &c.outline, &c.class_options);
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
        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![6];
        c.jobname = String::from("lecture-6");
        c.class_options = vec![
            String::from("nopresentation"),
            String::from("12pt")];

        let tex = get_tex_code(&c.range, &c.outline, &c.class_options);
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
        let mut c = compile::CMD::new();
        c.outline = o;
        c.range = vec![4];
        c.jobname = String::from("out");

        let tex = get_tex_code(&c.range, &c.outline, &c.class_options);

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
        let v = compile::generate_range_from_name(arg, &o);

        let arg2 = "s2";
        let v2 = compile::generate_range_from_name(arg2, &o);

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
        let v = compile::generate_range_from_name(arg, &o);

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
        let v = compile::generate_range_from_name(arg, &o);

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
        let v = compile::generate_range_from_name(arg, &o);

        assert_eq!(v, vec![1,2,3,4,5,6,7,8,9,10]);
    }

}
