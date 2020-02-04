use std::collections::HashMap;

pub struct RawArguments {
    // Flags with values attached to it: -flag value -flag2 value2
    pub args: HashMap<String, String>,

    // Standalone values: a b c d
    pub values: Vec<String>,

    // Standalone flags: -flag1 --flag2
    pub flags: Vec<String>
}

/**
 * Parses command-line-arguments and returns a map with values, arguments and flags.
 */
pub fn parse_raw_args<T: Iterator<Item = String>>(raw_args: T) -> RawArguments {
    let mut args: HashMap<String, String> = HashMap::new();
    let mut values: Vec<String> = Vec::new();
    let mut flags: Vec<String> = Vec::new();

    // Read arguments
    let mut iter = raw_args
        .skip(1) // skip first argument // which is always the executable itself
        .peekable();

    while iter.peek().is_some() {
        let value = iter.next().unwrap();

        // If the value starts with a - it's a flag / argument
        if value.starts_with("-") {
            // There may be a value attached to it
            match iter.peek() {
                Some(str) => {
                    if !str.starts_with("-") {
                        // Save argument
                        args.insert(value, iter.next().unwrap());
                        continue;
                    }
                }
                None => ()
            }

            // Push flag
            flags.push(value);
            continue;
        }

        // Simple value
        values.push(value);
    }

    return RawArguments {
        args,
        values,
        flags
    };
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::utils::cli_utils::parse_raw_args;

    #[test]
    fn parse_nothing() {
        let raw_args: Vec<String> = Vec::new();
        let ex_args: HashMap<String, String> = HashMap::new();
        let ex_values: Vec<String> = Vec::new();
        let ex_flags: Vec<String> = Vec::new();

        let args = parse_raw_args(raw_args.iter().map(|s| s.to_string()));
        assert_eq!(args.args, ex_args);
        assert_eq!(args.values, ex_values);
        assert_eq!(args.flags, ex_flags);
    }

    #[test]
    fn parse_values() {
        let raw_args: Vec<&str> = vec!["_", "source", "index"];
        let ex_args: HashMap<String, String> = HashMap::new();
        let ex_flags: Vec<String> = Vec::new();

        let args = parse_raw_args(raw_args.iter().map(|s| s.to_string()));
        assert_eq!(args.args, ex_args);
        assert_eq!(args.values, ["source", "index"]);
        assert_eq!(args.flags, ex_flags);
    }

    #[test]
    fn parse_flags() {
        let raw_args: Vec<&str> = vec!["_", "-source", "index", "-target", "t-t-target"];
        let mut ex_args: HashMap<String, String> = HashMap::new();
        ex_args.insert(String::from("-source"), String::from("index"));
        ex_args.insert(String::from("-target"), String::from("t-t-target"));

        let ex_values: Vec<String> = Vec::new();
        let ex_flags: Vec<String> = Vec::new();

        let args = parse_raw_args(raw_args.iter().map(|s| s.to_string()));
        assert_eq!(args.args, ex_args);
        assert_eq!(args.values, ex_values);
        assert_eq!(args.flags, ex_flags);
    }

    #[test]
    fn parse_mixed() {
        let raw_args: Vec<&str> = vec![
            "_", "source", "source2", "-flag1", "-arg2", "arg-val2", "-flag2", "-a", "wo",
        ];
        let mut ex_args: HashMap<String, String> = HashMap::new();
        ex_args.insert(String::from("-arg2"), String::from("arg-val2"));
        ex_args.insert(String::from("-a"), String::from("wo"));

        let args = parse_raw_args(raw_args.iter().map(|s| s.to_string()));
        assert_eq!(args.args, ex_args);
        assert_eq!(args.values, ["source", "source2"]);
        assert_eq!(args.flags, ["-flag1", "-flag2"]);
    }
}
