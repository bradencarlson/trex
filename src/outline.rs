
const DEFAULT_PREAMBLE: &str = "\\documentclass{article}
\\usepackage{amsmath,amstext,amssymb}\n";

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
