/* config.rs
 * 
 * Author: Braden Carlson
 * Date: September 2025
 *
 * Functions to parse in the configuration file. Only the read() function is
 * public.
 */

use std::fs;
use std::io;

const DEFAULT_PREAMBLE: &str = "\\documentclass{article}
\\usepackage{amsmath,amstext,amssymb}\n";

pub const DEFAULT_CONFIG: &str = "trex.conf";

const MINIMAL_CONFIG: &str = "preamble: preamble.tex
section: Section Label
file: file-1.tex";

#[derive(Debug)]
pub struct Outline {
    /* Holds the outline of the lecture notes. */

    /* This is a list of file names, in the order in which they appeared 
    * in the config file. */
    lecture_files: Vec<String>,

    /* This the filename of the preamble document. */
    preamble: String,

    /* This is a list of indices (in the lecture_files vector) at which new
    * sections start. For example, if 3 is an element of this vector, then
    * before the 4th file in lecture_files, a new section should be defined. */
    section_positions: Vec<usize>,

    /* This is a list of the section names, in the order in which they
     * appeared in the config file. */
    section_names: Vec<String>,
}

impl Outline {
    pub fn get_preamble(&self) -> String {
        if self.preamble.len() > 0{
            let mut p = String::new();
            p.push_str("\\include{");
            p.push_str(self.preamble.as_str());
            p.push_str("}\n");
            p
        } else {
            DEFAULT_PREAMBLE.to_string()
        }
    }

    pub fn get_full(&self) -> String {
        let mut p = String::new();
        p.push_str("\\begin{document}\n");
        let mut idx: usize = 0;
        let mut section_idx: usize = 0;
        for file in &self.lecture_files {
            if self.section_positions.contains(&idx) {
                p.push_str("\\section{");
                p.push_str(self.section_names[section_idx].as_str());
                p.push_str("}\n");
                section_idx += 1;
            }
            p.push_str("\\input{");
            p.push_str(file.as_str());
            p.push_str("}\n");

            idx += 1;
        }
        p
    }

    pub fn get_range(&self, range: &Vec<usize>) -> String {
        let mut p = String::new();
        p.push_str("\\begin{document}\n");
        let mut section_counter = 0;
        let mut section_idx = 0;

        /* Figure out how many sections we need to skip to get to the first 
         * lecture in the range. */
        loop {
            if section_idx < self.section_positions.len() && 
                self.section_positions[section_idx] <= range[0] {
                section_idx += 1;
                section_counter += 1;
            } else {
                break;
            }
        }

        p.push_str("\\section{");
        p.push_str(self.section_names[section_idx-1].as_str());
        p.push_str("}\n");

        for idx in range {
            if section_idx < self.section_positions.len() && 
                idx == &self.section_positions[section_idx] {
                    p.push_str("\\section{");
                    p.push_str(self.section_names[section_idx].as_str());
                    p.push_str("}\n");
                    section_idx += 1;

            }

            p.push_str("\\input{");
            p.push_str(self.lecture_files[*idx-1].as_str());
            p.push_str("}\n");
        }

        p
    }
}


pub fn read(path: &str) -> Option<Outline> {
    /* Reads the confige file and parses the result into an Outline struct.
     * 
     * Parameters: 
     *  path - the filepath to read. 
     *
     * Returns: 
     *  Option<Outline> - The outline of the lecture notes is reading the config file is
     *                               succesful. An Error if it is not.
     */

    let config = match fs::read_to_string(&path) {
        Ok(string) => string, 
        Err(_) => return None,
     };

    parse(&config)
}

fn parse(content: &String) -> Option<Outline> {
    /* Parses the string passed line by line looking for keywords that should
     * be found in the config file. This method looks for string of the form 
     *
     *  keyword: value
     *
     * and if the keyword is valid, value is added to the Outline structure
     * to be returned to the caller. 
     *
     * Parameters: 
     *  content - The string to parse. It is ok for content to posses newline
     *            characters, in fact, it is expected. 
     *
     * Returns: 
     *  Outline - an Outline struct containing the information about the
     *            outline of the lecture notes. 
     */

    let mut lines = content.lines();

    let mut outline = Outline {
        lecture_files: Vec::<String>::new(),
        preamble: String::new(),
        section_positions: Vec::<usize>::new(),
        section_names: Vec::<String>::new()
    };

    loop {
        let line = match lines.next() {
            Some(line) => line.trim(),
            None => break,
        };

        let preamble = "preamble: ";
        let section = "section: ";
        let file = "file: ";

        if line.starts_with(preamble) {
            let arg = &line[preamble.len()..];
            outline.preamble.push_str(arg);
        } else if line.starts_with(section) {
            let arg = &line[section.len()..];
            outline.section_names.push(arg.to_string());
            outline.section_positions.push(
                outline.lecture_files.len().try_into()
                    .expect("Number of lecture files should not be this large")
            );
        } else if line.starts_with(file) {
            let arg = &line[file.len()..];
            outline.lecture_files.push(arg.to_string());
        }
    }

    Some(outline)
}

