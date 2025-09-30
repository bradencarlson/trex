
use trex::parse_args;

#[test]
fn filename_arg() {
    let k = String::from("-f");
    let v = String::from("value");
    assert!(parse_args::parse_argument(&k,&v));
}

