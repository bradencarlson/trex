use std::collections::HashMap;
use std::fs;

pub const DEFAULT_CONFIG: &str = "test_file.txt";

pub fn read(path: &str) -> HashMap<String, String> {
    let options = match fs::read_to_string(&path) {
        Ok(content) => parse(&content),
        Err(_) => create_config_and_parse(),
    };

    options
}

fn parse(content: &String) -> HashMap<String, String> {
    let mut options = HashMap::new();

    let lines = content.lines();

    options
}

fn create_config_and_parse() -> HashMap<String, String> {
    let mut options = HashMap::new();
    options
}
