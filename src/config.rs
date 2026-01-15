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

/* This is the default config file, if no filepath is passed to the read 
 * function, this will be searched for. */
pub const DEFAULT_CONFIG: &str = "trex.conf";

/* If no config file is found, this default content is parsed. */
const MINIMAL_CONFIG: &str = "preamble: preamble.tex
section: Section Label
file: file-1.tex";


pub fn read(path: &str) -> Option<Outline> {
    /* Reads the config file and parses the result into an Outline struct.
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
    let mut outline = Outline::new(); 
    let mut file_counter = 0;
    let mut chapter_counter = 0;
    let mut section_counter = 0;
    let mut last_chapter_name = String::new();
    let mut last_section_name = String::new();

    outline.chapter_positions.push(0);
    outline.chapter_names.push(String::new());
    outline.section_positions.push(0);
    outline.section_names.push(String::new());

    loop {
        let line = match lines.next() {
            Some(line) => line.trim(),
            None => break,
        };

        // Define the keywords to match against.

        // These are options
        let preamble = "preamble: ";
        let class = "class: ";
        let handle_subsections = "handle_subsections: ";
        let handle_sections = "handle_sections: ";

        // These form the actual structure of the lecture notes.
        let section = "section: ";
        let chapter = "chapter: ";
        let file = "file: ";

        // Others
        let comment = "#";


        if line.starts_with(preamble) {
            let arg = &line[preamble.len()..];
            outline.preamble.push_str(arg);
        } else if line.starts_with(section) {
            let arg = &line[section.len()..];
            outline.section_names[file_counter] = arg.to_string();
            outline.section_positions[file_counter] = 1;
            last_section_name = arg.to_string();
            section_counter += 1;
        } else if line.starts_with(chapter) {
            let arg = &line[chapter.len()..];
            outline.chapter_names[file_counter] = arg.to_string();
            outline.chapter_positions[file_counter] = 1;
            last_chapter_name = arg.to_string();
            chapter_counter += 1;
            section_counter = 0;
        } else if line.starts_with(file) {
            let arg = &line[file.len()..];
            outline.files.push(arg.to_string());
            outline.chapter_positions.push(0);
            outline.section_positions.push(0);
            outline.chapter_names.push(last_chapter_name.clone());
            outline.section_names.push(last_section_name.clone());
            outline.chapter_indices.push(chapter_counter);
            outline.section_indices.push(section_counter);
            file_counter += 1;
        } else if line.starts_with(class) {
            let arg = &line[class.len()..];
            outline.class = arg.to_string();
        } else if line.starts_with(handle_subsections) {
            let arg = &line[handle_subsections.len()..];
            if arg == "true" {
                outline.handle_subsections = true;
            } else {
                outline.handle_subsections = false;
            }
        } else if line.starts_with(comment) {
            // Don't parse this line.
            continue;
        } else if line.starts_with(handle_sections) {
            let arg = &line[handle_sections.len()..];
            if arg == "true" {
                outline.handle_sections = true;
            } else {
                outline.handle_sections = false;
            }
        }
    }

    Some(outline)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> String {
        "class: article
        preamble: preamble.tex

        chapter: Chapter One
        section: Section One
            file: file-1.tex
            file: file-2.tex
            file: file-3.tex
        section: Section Two
            file: file-4.tex
        chapter: Chapter Two
            file: file-5.tex".to_string()
    }

    #[test]
    fn test_config() {
        let out = parse(&default_config()).unwrap();
        assert_eq!(out.preamble, "preamble.tex");
        assert_eq!(out.class, "article");
        assert_eq!(out.section_names[0], "Section One");
        assert_eq!(out.section_names[1], "Section Two");
        assert_eq!(out.section_positions[0], 0);
        assert_eq!(out.section_positions[1], 3);
        assert_eq!(out.chapter_names[0], "Chapter One");
        assert_eq!(out.chapter_names[1], "Chapter Two");
        assert_eq!(out.chapter_positions[0], 0);
        assert_eq!(out.chapter_positions[1], 4);
    }
}
