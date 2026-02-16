/* config.rs
 *
 * Author: Braden Carlson
 * Date: September 2025
 *
 * Functions to parse in the configuration file. Only the read() function is
 * public.
 */

use std::fs;
use std::fmt;

/* This is the default config file, if no filepath is passed to the read
 * function, this will be searched for. */
pub const DEFAULT_CONFIG: &str = "trex.conf";

/* If no config file is found, this default content is parsed. */
const MINIMAL_CONFIG: &str = "preamble: preamble.tex
section: Section Label
file: file-1.tex";


/* Outline struct which will hold all the information about the config file, including the list of
 * chapter/section names, files given, class information and more. */
#[derive(Debug)]
pub struct Outline {
    /* Holds the outline of the lecture notes. */

    /* This is a list of file names, in the order in which they appeared
    * in the config file. */
    pub files: Vec<String>,

    /* This the filename of the preamble document. */
    pub preamble: String,

    /* Holds the class of the document */
    pub class: String,

    /* Whether or not chapter commands should be done by the TreX. */
    pub handle_chapters: bool,

    /* Whether or not section commands should be done by the TreX. */
    pub handle_sections: bool,

    /* Whether or not subsection commands should be done by TreX. */
    pub handle_subsections: bool,

    /* List of positions where chapter commands appear. Similar to section_positions */
    pub chapter_positions: Vec<usize>,

    /* This is a list of the chapter names, in the order in which they
     * appeared in the config file. */
    pub chapter_names: Vec<String>,

    /* This keeps track of the chapter numbers as they should appear
    * in the final pdf. */
    pub chapter_indices: Vec<i32>,

    /* This is a list of indices (in the files vector) at which new
    * sections start. For example, if there is a section command before the 3rd file
    * listed in the config file, then there will be a 1 in index 2 in this array. */
    pub section_positions: Vec<usize>,

    /* This is a list of the section names, in the order in which they
     * appeared in the config file. */
    pub section_names: Vec<String>,

    /* This keeps track of section numbers as they should appear in the final pdf. */
    pub section_indices: Vec<i32>,

    /* Keeps track of the current subsection index which should be used, if handle_subsections
     * is set to true. */
    pub subsection_indices: Vec<i32>,

    /* The bibliography style, if any */
    pub bib_style: String,

    /* The bibliography file, if any */
    pub bib_file: String


}

impl fmt::Display for Outline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* Prints out information about the outline, including the total number of sections found,
         * location of the preamble file, and all sections and lecture files found, in the order
         * that they were found in.
         */

        write!(f, "preamble: {}\n", self.preamble);
        write!(f, "Number of Sections: {}\n", self.section_positions.len());
        write!(f, "handle_subsections: {}\n", self.handle_subsections);
        let mut c_idx = 0;
        let mut s_idx = 0;
        let mut idx = 0;

        write!(f, "\nchapter positions: {:?}", self.chapter_positions);
        write!(f, "\nchapter names: {:?}", self.chapter_names);
        write!(f, "\nchapter indices: {:?}", self.chapter_indices);
        write!(f, "\nsection positions: {:?}", self.section_positions);
        write!(f, "\nsection names: {:?}", self.section_names);
        write!(f, "\nsection indices: {:?}", self.section_indices);
        write!(f, "\nsubsection indices: {:?}", self.subsection_indices)


    }
}

impl Outline {
    pub fn new() -> Outline {
        /* Creates a new Outline, with empty fields. */

        Outline {
            files: Vec::<String>::new(),
            preamble: String::new(),
            class: String::new(),
            handle_chapters: true,
            handle_sections: true,
            handle_subsections: false,
            chapter_positions: Vec::<usize>::new(),
            chapter_names: Vec::<String>::new(),
            chapter_indices: Vec::<i32>::new(),
            section_positions: Vec::<usize>::new(),
            section_names: Vec::<String>::new(),
            section_indices: Vec::<i32>::new(),
            subsection_indices: Vec::<i32>::new(),
            bib_style: String::new(),
            bib_file: String::new()
        }
    }
    pub fn get_preamble(&self) -> Option<&String> {
        /* Returns the preamble if it is nonzero, otherwise returns None */

        if self.preamble.len() > 0 {
            Some(&self.preamble)
        } else {
            None
        }
    }
    pub fn get_lecture(&self, index: usize) -> Option<&String> {
        /* Returns the lecture file at the given index, if it exists.
         *
         * Parameters:
         *  index   - the index of the lecture file to get.
         *
         * Returns:
         *  Some(String) - if the index is valid
         *  None         - if the index is invalid
         */

        self.files.get(index)
    }
    pub fn handle_subsections(&self) -> bool {
        /* Returns the value of the handle_subsections parameter. */
        self.handle_subsections
    }
}

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
    let mut subsection_counter  = 0;
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
        let bib_style = "bib_style: ";
        let bib_file = "bibliography: ";

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
            subsection_counter = 0;
        } else if line.starts_with(chapter) {
            let arg = &line[chapter.len()..];
            outline.chapter_names[file_counter] = arg.to_string();
            outline.chapter_positions[file_counter] = 1;
            last_chapter_name = arg.to_string();
            chapter_counter += 1;
            section_counter = 0;
            subsection_counter = 0;
        } else if line.starts_with(file) {
            let arg = &line[file.len()..];
            outline.files.push(arg.to_string());
            outline.chapter_positions.push(0);
            outline.section_positions.push(0);
            outline.chapter_names.push(last_chapter_name.clone());
            outline.section_names.push(last_section_name.clone());
            outline.chapter_indices.push(chapter_counter);
            outline.section_indices.push(section_counter);
            outline.subsection_indices.push(subsection_counter);
            subsection_counter += 1;
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
        } else if line.starts_with(bib_style) {
            let arg = &line[bib_style.len()..];
            outline.bib_style = String::from(arg);
        } else if line.starts_with(bib_file) {
            let arg = &line[bib_file.len()..];
            outline.bib_file = String::from(arg);
        }
    }

    Some(outline)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let out = match read(
            "examples/default.conf") {
            Some(outline) => outline,
            None => Outline::new(),
        };
        assert_eq!(out.preamble, "preamble.tex");
        assert_eq!(out.class, "article");

        let mut idx = 0;
        let num_files = out.files.len();

        loop {
            if idx >= num_files {
                break;
            }

            /* Check to make sure the chapter indices and names are correct */
            if idx <= 6 {
                assert_eq!(out.chapter_names[idx], "Chapter 1");
                assert_eq!(out.chapter_indices[idx], 1);
            } else {
                assert_eq!(out.chapter_names[idx], "Chapter 2");
                assert_eq!(out.chapter_indices[idx], 2);
            }

            /* Check to make sure that the section indices and names are correct. */
            if idx <= 2 {
                assert_eq!(out.section_names[idx], "Files 1-3");
                assert_eq!(out.section_indices[idx], 1);
            } else if idx <= 6 {
                assert_eq!(out.section_names[idx], "Files 4-7");
                assert_eq!(out.section_indices[idx], 2);
            } else {
                assert_eq!(out.section_names[idx], "Files 8-10");
                assert_eq!(out.section_indices[idx], 1);
            }

            /* Make sure that the chapter_positions are correct. */
            if idx == 0 || idx == 7 {
                assert_eq!(out.chapter_positions[idx], 1);
            } else {
                assert_eq!(out.chapter_positions[idx], 0);
            }

            /* Finally, make sure that the section_positions are correct. */
            if idx == 0 || idx == 3 || idx == 7 {
                assert_eq!(out.section_positions[idx], 1);
            } else {
                assert_eq!(out.section_positions[idx], 0);
            }

            idx += 1;
        }
    }
}
