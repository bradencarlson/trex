
use trex::config;

#[test]
fn default_config_test() {
    let outline = config::read("dne.txt").unwrap();
    let preamble = match outline.get_preamble() {
        Some(s) => s, 
        None => "none",
    };

    let file = match outline.get_lecture(0) {
        Some(s) => s, 
        None => "none",
    };

    let none = match outline.get_lecture(1) {
        Some(s) => s, 
        None => "failed",
    };

    assert_eq!(preamble, "preamble.tex");
    assert_eq!(file, "file-1.tex");
    assert_eq!(none, "failed");

}

fn config_test() {
    let outline = config::read("sample_config.txt").unwrap();
    let preamble = match outline.get_preamble() {
        Some(s) => s, 
        None => "failed",
    };
    let file_one = outline.get_lecture(0)
        .expect("Sample config should have a file.");

    let file_two = outline.get_lecture(1)
        .expect("Sample config should have a file here.");

    let file_three = outline.get_lecture(2)
        .expect("Sample config should have a file here.");

    let num_sections = outline.section_positions.len();
    let num_sec_names = outline.section_names.len();

    assert_eq!(preamble, "testing.tex");
    assert_eq!(file_one, "first-lecture.tex");
    assert_eq!(file_two, "second-lecture.tex");
    assert_eq!(file_three, "third-lecture.tex");
    assert_eq!(num_sections, 2);
    assert_eq!(num_sec_names, 2);
}
