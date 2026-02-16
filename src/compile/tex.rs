use crate::config::Outline;

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
