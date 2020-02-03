use std::collections::HashMap;
use std::env::Args;

pub struct RawArguments {
    pub  args: HashMap<String, String>,
    pub values: Vec<String>,
    pub flags: Vec<String>,
}

pub fn parse_raw_args(raw_args: Args) -> RawArguments {
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
        flags,
    };
}