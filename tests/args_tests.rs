
use trex::parse_args;

#[test]
fn filename_arg() {
    let k = String::from("-f");
    let v = String::from("value");
    assert!(parse_args::parse_argument(&k,&v));
}

#[test]
fn range_arg() {
    let r = String::from("-r");
    let v = String::from("1-3");
    assert!(parse_args::parse_argument(&r, &v));
}

#[test]
fn engine_arg() {
    let k = String::from("-e");
    let v = String::from("pdflatex");
    assert!(parse_args::parse_argument(&k, &v));
}
