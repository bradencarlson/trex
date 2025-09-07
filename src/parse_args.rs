use std::collections::HashMap;

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

        if parse_argument(&key, &value) {
            map.insert(key, value);
        } 
        
        idx += 2;

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
        String::from("-f"),
        String::from("-b")
    ];

    valid_args.contains(key)
}
