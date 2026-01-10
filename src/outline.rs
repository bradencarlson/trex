/* outline.rs
 *
 * Author: Braden Carlson
 * Date: September 2025
 *
 * Provides the Outline struct, which is used to hold the structure of the 
 * document which is to be compiled. 
 */

use std::fmt;

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

    /* Whether or not subsection numbers should be done by TreX. */
    pub handle_subsections: bool,

    /* Whether or not subsection numbers should be done by the TreX. */
    pub handle_sections: bool,

    /* This is a list of indices (in the files vector) at which new
    * sections start. For example, if 3 is an element of this vector, then
    * before the 4th file in files, a new section should be defined. */
    pub section_positions: Vec<usize>,

    /* This is a list of the section names, in the order in which they
     * appeared in the config file. */
    pub section_names: Vec<String>,

    /* This is a list of indices (in the files vector) at which new
    * chapters start. For example, if 3 is an element of this vector, then
    * before the 4th file in files, a new chapter should be defined. */
    pub chapter_positions: Vec<usize>,

    /* This is a list of the chapter names, in the order in which they
     * appeared in the config file. */
    pub chapter_names: Vec<String>,
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
        let mut s_idx = 0;
        let mut idx = 0;
        loop {
            if idx >= self.files.len() {
                break;
            }
            if s_idx < self.section_positions.len() && 
                idx == *self.section_positions.get(s_idx).unwrap() {
                    write!(f, "section: {}\n", self.section_names.get(s_idx).unwrap());
                    s_idx += 1;
            }
            write!(f, "\tfile: {}\n", self.files.get(idx).unwrap());
            idx += 1;
        }
        write!(f, "\nsection positions: {:?}", self.section_positions)
    }
}

impl Outline {
    pub fn new() -> Outline {
        /* Creates a new Outline, with empty fields. */

        Outline {
            files: Vec::<String>::new(), 
            preamble: String::new(), 
            class: String::new(),
            handle_subsections: false,
            handle_sections: true,
            section_positions: Vec::<usize>::new(),
            section_names: Vec::<String>::new(),
            chapter_positions: Vec::<usize>::new(),
            chapter_names: Vec::<String>::new(),
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
