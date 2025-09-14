/* config.rs
 * 
 * Author: Braden Carlson
 * Date: September 2025
 *
 * Functions to parse in the configuration file. Only the read() function is
 * public.
 */

use std::fs;

use crate::outline::Outline;


pub const DEFAULT_CONFIG: &str = "trex.conf";

const MINIMAL_CONFIG: &str = "preamble: preamble.tex
section: Section Label
file: file-1.tex";


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
        Err(_) => MINIMAL_CONFIG.to_string(),
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

