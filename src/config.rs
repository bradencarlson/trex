use std::collections::HashMap;
use std::fs;
use toml::Table;

pub const DEFAULT_CONFIG: &str = "trex.toml";
const MINIMAL_CONFIG: &str = "[files]
1 = test.tex";

pub fn read(path: &str) -> Option<Table>  {
    match fs::read_to_string(&path) {
        Ok(content) => parse(&content),
        Err(_) => create_config_and_parse(),
    }
}

fn parse(content: &String) -> Option<Table> {
    match toml::from_str(content) {
        Ok(table) => table,
        Err(_) => None,
    }
}

fn create_config_and_parse() -> Option<Table> {
    match toml::from_str(MINIMAL_CONFIG) {
        Ok(table) => table,
        Err(_) => None
    }
}

