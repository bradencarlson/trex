
use trex::config;
use trex::outline::Outline;

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
