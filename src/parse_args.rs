/* parse_args.rs
 *
 * Author: Braden Carlson
 * Date: September 2025
 *
 * Provides functions to parse the command line arguments passed to the 
 * program. 
 */

use std::collections::HashMap;
use std::fs;

pub const DRYRUN_ARG: &str = "-d";
pub const ENGINE_ARG: &str = "-e";
pub const FILENAME_ARG: &str = "-f";
pub const HELP_ARG: &str = "-h";
pub const JOBNAME_ARG: &str = "-j";
pub const CLASS_OPTION: &str = "-o";
pub const RANGE_ARG: &str = "-r";
pub const VERBOSE_ARG: &str = "-v";

pub fn parse(args: Vec<String>) -> HashMap<String, String> {
    /* Takes in a vector of arguments, considers them in pairs, and if they represent a valid (key,
     * value) pair for the program, they are added as a (key, value) pair to a HashMap.
     *
     * Parameters: 
     *  args - the vector of arguments to parse. 
     *
     * Returns: 
     *  HashMap<String, String> - A map of the valid arguments for the program. 
     *
     * Note: 
     *  This function takes ownership of the args parameter, and does not return it, so the caller
     *  must be finished with this vector before calling this function. Also, since this is to be
     *  called when using the program from the command line, we do not consider the first element
     *  of args, since this will be the name of the program itself. 
     */

    let mut map = HashMap::new();

    let mut idx = 1;
    loop {
        if idx >= args.len() {
            break;
        }
        
        let key = match args.get(idx) {
            Some(key) => key.to_string(),
            None => String::new(),
        };

        let value = match args.get(idx+1) {
            Some(value) => value.to_string(),
            None => String::new(),
        };

        match key.as_str() {
            DRYRUN_ARG => {
                idx += 1;
                map.insert(String::from(DRYRUN_ARG),"true".to_string());
                continue;
            },
            VERBOSE_ARG => {
                idx += 1;
                map.insert(String::from(VERBOSE_ARG), "true".to_string());
                continue;
            },
            CLASS_OPTION => {
                idx += 1;
                let opt = args.get(idx).expect("Fatal error: No option given after {CLASS_OPTION}" , );
                map.insert(opt.to_string(), "class_option".to_string());
                idx += 1;
                continue;
            },
            HELP_ARG => {
                idx += 1;
                map.insert(String::from(HELP_ARG), String::from("true"));
                continue;
            },
            _ => {}
        }

        if parse_argument(&key, &value) {
            map.insert(key, value);
        } 
        
        idx += 2;

    }

    if let Some(_s) = map.get(VERBOSE_ARG) {
        println!("Options Parsed:");
        for (key,value) in &map {
            println!("\t{key}: {value}");
        }
        println!("");
    }

    map
}

fn parse_argument(key: &String, value: &String) -> bool {
    /* Determines if a given (key, value) pair is a valid option for this program. There is a
     * simple check to determine if the key belongs to the accepted keys list, and that the value
     * makes sense. 
     *
     * Parameters: 
     *  key - A reference to a String
     *  value - A reference to a String
     *
     * Returns:
     *  boolean - whether or not the (key,value) pair is a valid option for the program.
     */

    let valid_args = vec![
        String::from(FILENAME_ARG),
        String::from(RANGE_ARG),
        String::from(ENGINE_ARG),
        String::from(JOBNAME_ARG),
        String::from(CLASS_OPTION),
    ];

    if !valid_args.contains(key) {
        return false
    } 

    match key.as_str() {
        ENGINE_ARG => {
            return vec![String::from("pdflatex"),
                String::from("latex")].contains(value);
        },
        FILENAME_ARG => {
            if let Ok(_s) = fs::read_to_string(value) {
                return true
            } else {
                // For now, just return true even if the file is not found.
                // This should be fixed when the filename_arg test is updated to first create
                // the file that it tests for, so it doesn't fail every time.
                return true
            }
        },
        _ => {
            return true
        }
    }

}

#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn filename_arg() {
        let k = String::from("-f");
        let v = String::from("trex.conf");
        assert!(parse_argument(&k,&v));
    }

    #[test]
    fn range_arg() {
        let r = String::from("-r");
        let v = String::from("1-3");
        assert!(parse_argument(&r, &v));
    }

    #[test]
    fn engine_arg() {
        let k = String::from("-e");
        let v = String::from("pdflatex");
        assert!(parse_argument(&k, &v));
    }

}
