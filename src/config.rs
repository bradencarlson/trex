use std::collections::HashMap;
use std::fs;
use toml::Table;

pub const DEFAULT_CONFIG: &str = "trex.toml";
const MINIMAL_CONFIG: &str = "[files]
1 = test.tex";

pub fn read(path: &str) -> Table  {
    match fs::read_to_string(&path) {
        Ok(content) => parse(&content),
        Err(_) => create_config_and_parse(),
    }
}

fn parse(content: &String) -> Table {
    let result = toml::from_str(content);
    match result {
        Ok(table) => table,
        Err(_) => {
            std::process::exit; 
            toml::from_str(MINIMAL_CONFIG).expect("")
        },
    }
}

fn create_config_and_parse() -> Table {
    toml::from_str(MINIMAL_CONFIG).expect("")
}

