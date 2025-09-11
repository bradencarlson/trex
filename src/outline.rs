use std::fmt;

#[derive(Debug)]
pub struct Outline {
    /* Holds the outline of the lecture notes. */

    /* This is a list of file names, in the order in which they appeared 
    * in the config file. */
    pub lecture_files: Vec<String>,

    /* This the filename of the preamble document. */
    pub preamble: String,

    /* This is a list of indices (in the lecture_files vector) at which new
    * sections start. For example, if 3 is an element of this vector, then
    * before the 4th file in lecture_files, a new section should be defined. */
    pub section_positions: Vec<usize>,

    /* This is a list of the section names, in the order in which they
     * appeared in the config file. */
    pub section_names: Vec<String>,
}

impl fmt::Display for Outline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "preamble: {}\n", self.preamble);
        write!(f, "Number of Sections: {}\n", self.section_positions.len());
        let mut s_idx = 0;
        let mut idx = 0;
        loop {
            if idx >= self.lecture_files.len() {
                break;
            }
            if s_idx < self.section_positions.len() && 
                idx == *self.section_positions.get(s_idx).unwrap() {
                    write!(f, "section: {}\n", self.section_names.get(s_idx).unwrap());
                    s_idx += 1;
            }
            write!(f, "\tfile: {}\n", self.lecture_files.get(idx).unwrap());
            idx += 1;
        }
        write!(f, "\nsection positions: {:?}", self.section_positions)
    }
}

impl Outline {
    pub fn new() -> Outline {
        Outline {
            lecture_files: Vec::<String>::new(), 
            preamble: String::new(), 
            section_positions: Vec::<usize>::new(),
            section_names: Vec::<String>::new(),
        }
    }
    pub fn get_preamble(&self) -> Option<&String> {
        if self.preamble.len() > 0 {
            Some(&self.preamble)
        } else {
            None
        }
    }
    pub fn get_lecture(&self, index: usize) -> Option<&String> {
        self.lecture_files.get(index)
    }
}
