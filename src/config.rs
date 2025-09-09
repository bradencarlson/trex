use std::collections::HashMap;
use std::fs;
use toml::Table;
use std::io;

struct Section {
    number: u8,
    name: String, 
    files: Vec<String>,
}

pub const DEFAULT_CONFIG: &str = "trex.toml";
const MINIMAL_CONFIG: &str = "[files]
1 = test.tex";

pub fn read(path: &str) -> Result<String, io::Error>  {
    let config = fs::read_to_string(&path)?; 

    Ok(config)

}

fn parse(content: &String) -> Table {
    let config = match toml::from_str(content) {
        Ok(table) => table,
        Err(error) => {
            println!("{error}");
            Table::new()
        },
    };


    for (key, value) in &config {
        if key.as_str().contains("section") {
            println!("{value:?}");
        }
    }

    config
}

fn parse_section_table(section: Table)  {
    let mut name = String::new();
    let mut number: u8 = 0;
    let mut files = Vec::<String>::new();

    for (key, value) in &section {
        let key_str = key.as_str();
        if key_str.contains("file") {
            let file = value.as_str().expect(
                "Something went wrong while parsing config file."
            );
            files.push(file.to_string());
        } else if key_str.contains("number") {
            let n = value.as_str().expect(
                "Something went wrong while parsing config file."
            );
            number = match str::parse(n) {
                Ok(num) => num, 
                Err(_) => 0
            };
        } else if key_str.contains("name") {
            let nm = value.as_str().expect(
                "Something went wrong while parsing config file."
            );
            name.push_str(nm);
        }

    }

    println!("name: {name}");
    println!("number: {number}");
    println!("files: {files:?}");

}

fn create_config_and_parse() -> Table {
    match toml::from_str(MINIMAL_CONFIG) {
        Ok(table) => table,
        Err(_) => Table::new(),
    }
}

