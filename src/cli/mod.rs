use std::collections::HashMap;

use flag::CLIFlag;
use result::CLIResult;
use value::CLIValue;

use crate::lib::Wrapping;

pub mod flag;
pub mod result;
pub mod value;

pub struct CLIApp {
    name: String,
    flags: Vec<CLIFlag>,
    values: Vec<CLIValue>,
}

impl CLIApp {
    /// Creates a new cli-application
    pub fn new() -> Self {
        CLIApp {
            name: String::new(),
            flags: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Sets a name for this app
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Defines a flag
    /// Panics if name or one of the abbreviations is already in use.
    pub fn add_flag(mut self, new_flag: CLIFlag) -> Self {
        // Check for duplicates
        for flag in &self.flags {
            // Check name
            if flag.name.eq(&new_flag.name) {
                panic!("Flag with name \"{}\" is already defined.", &new_flag.name)
            }

            // Check if abbreviation is already in use
            for abbr in &flag.abbr {
                if new_flag.abbr.contains(abbr) {
                    panic!(
                        "Flag with name \"{}\" uses \"{}\" which is already in use by \"{}\".",
                        &new_flag.name, &abbr, &flag.name
                    )
                }
            }
        }

        self.flags.push(new_flag);
        self
    }

    /// Adds a new value
    /// Panics if the name is already taken
    pub fn add_value(mut self, val: CLIValue) -> Self {
        // Check if name is already in use
        if self
            .values
            .iter()
            .position(|v| v.name.eq(&val.name))
            .is_some()
        {
            panic!("Value with name \"{}\" is already defined.", &val.name)
        }

        self.values.push(val);
        self
    }

    /// Parses the cli-arguments
    pub fn consume_args(&self) -> Result<CLIResult, String> {
        self.consume(std::env::args())
    }

    /// Parses a list of arguments
    pub fn consume<T: Iterator<Item = String>>(&self, raw_args: T) -> Result<CLIResult, String> {
        let mut args: HashMap<String, String> = HashMap::new();
        let mut values: HashMap<String, String> = HashMap::new();
        let mut flags: Vec<String> = Vec::new();
        let mut value_offset = 0;
        let max_values = self.values.len();

        // Parse passed values
        let mut iter = raw_args
            .skip(1) // skip first argument // which is always the executable itself
            .peekable();

        while iter.peek().is_some() {
            let arg = iter.next().unwrap();

            // If the value starts with a - it's a flag / argument
            if arg.starts_with("-") {
                // Check if this is known flag
                let target_flag = match self.flags.iter().find(|flag| flag.has_abbr(&arg)) {
                    None => return Err(format!("Unknown flag: {}", arg)),
                    Some(flag) => flag,
                };

                // Copy name of flag
                let flag_name = target_flag.name.clone();

                // Parse value of flag if expected
                if target_flag.expects_value {
                    let mut had_value = false;

                    // TODO: Refactor
                    match iter
                        .peek()
                        .and_then(|s| {
                            if s.starts_with("-") {
                                Option::None
                            } else {
                                had_value = true;
                                Option::Some(s.clone())
                            }
                        })
                        .or(target_flag.resolve_default(&args))
                    {
                        None => return Err(format!("Flag {} expects a value.", arg)),
                        Some(val) => {
                            // Validate
                            match target_flag.validator {
                                None => (),
                                Some(v) => match v(&val) {
                                    Ok(_) => (),
                                    Err(e) => return Err(e),
                                },
                            };

                            args.insert(flag_name, val);

                            if had_value {
                                iter.next();
                            }

                            continue;
                        }
                    };
                }

                // Push flag
                flags.push(flag_name);
                continue;
            }

            // Check if too many values were passed
            if value_offset == max_values {
                return Err(format!(
                    "Too many values. Maximum is {} but got {} as last one.",
                    max_values, arg
                ));
            }

            // Save value
            values.insert(self.values.get(value_offset).unwrap().name.clone(), arg);

            // Increment value pointer
            value_offset += 1;
        }

        // Check if values are missing
        for val in &self.values {
            if values.contains_key(&val.name) {
                // Validate value
                match val.validator {
                    None => (),
                    Some(validator) => match validator(&val.name) {
                        Err(e) => return Err(e),
                        Ok(_) => (),
                    },
                }

                continue;
            }

            // Use default if not set
            match val.default {
                None => (),
                Some(def) => {
                    values.insert(val.name.clone(), def(&values));
                    continue;
                }
            }

            // Check if required but not set
            if val.required {
                return Err(format!("Missing value labeled \"{}\"", val.name));
            }
        }

        Ok(CLIResult::from(values, args, flags))
    }

    /// Creates a usage-description out of the currently defined attributes.
    pub fn usage_description(&self) -> String {
        let mut desc = format!("Usage: {}", self.name);

        // Push expected values to it
        for val in &self.values {
            desc.push_str(&format!(" <{}>", val.stringify().0));
        }

        // If flag were set indicate them with that little thingy
        match self.flags.len() {
            0 => (),
            1 => desc.push_str(" [options]"),
            _ => desc.push_str(" [options...]"),
        }

        desc
    }

    /// Prints a full help-text based on the previously defined attributes.
    pub fn print_help(&self) {
        // Print usage description
        println!("{}", self.usage_description());

        // Prepare flags for printing and aligning them
        let mut longest_left_side: usize = 0;
        let mut values_map: Vec<(String, String)> = Vec::new();
        let mut flag_map: Vec<(String, String)> = Vec::new();
        let mut arg_map: Vec<(String, String)> = Vec::new();

        for flag in &self.flags {
            let (usage, desc) = flag.stringify();

            // Update the maximum length of the command-syntax
            // This will be used to properly pad and align the commands later
            let len = usage.len();

            if len > longest_left_side {
                longest_left_side = len;
            }

            if flag.expects_value {
                arg_map.push((desc, usage));
            } else {
                flag_map.push((desc, usage));
            }
        }

        for val in &self.values {
            let (name, desc) = val.stringify();
            let len = name.len();

            if len > longest_left_side {
                longest_left_side = len;
            }

            values_map.push((desc, name.wrap_into("<", ">")));
        }

        // Print flags
        let sections = [
            ("Flags:", &flag_map),
            ("Arguments:", &arg_map),
            ("Values:", &values_map),
        ];

        for (section_name, content) in sections.iter() {
            if content.len() == 0 {
                continue;
            }

            println!("\n{}", section_name);
            for (name, flags) in content.iter() {
                println!("  {: <width$}  {}", flags, name, width = longest_left_side);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cli::flag::CLIFlag;
    use crate::cli::value::CLIValue;
    use crate::cli::*;

    macro_rules! create_args {
        ($($element: expr), *) => {
            {
                let mut v = Vec::new();
                $( v.push(String::from($element)); )*
                v.into_iter()
            }
        };
    }

    #[test]
    fn flags() {
        let app = CLIApp::new()
            .name("hello-world")
            .add_flag(CLIFlag::new("help").abbr("-h").abbr("--help"))
            .add_flag(CLIFlag::new("version").abbr("-v").abbr("--version"))
            .add_flag(
                CLIFlag::new("util")
                    .default(|_| String::from("hello"))
                    .abbr("-u")
                    .abbr("--util"),
            );

        let p1 = app
            .consume(create_args!["", "-h", "-u", "--version"])
            .unwrap();
        assert!(p1.has_flag("help"));
        assert!(p1.has_flag("version"));
        assert_eq!(p1.get_arg("util").unwrap(), "hello");

        let p2 = app
            .consume(create_args!("", "--util", "baz", "-h"))
            .unwrap();
        assert!(p2.has_flag("help"));
        assert!(!p2.has_flag("version"));
        assert_eq!(p2.get_arg("util").unwrap(), "baz");
    }

    #[test]
    fn fail_on_unknown_flags() {
        let app = CLIApp::new()
            .name("hello-world")
            .add_flag(CLIFlag::new("help").abbr("-h"));

        assert!(app.consume(create_args!("", "-s")).is_err());
        assert!(app.consume(create_args!("", "-h")).is_ok());
    }

    #[test]
    fn values() {
        let app = CLIApp::new()
            .name("hello-world")
            .add_value(
                CLIValue::new("source")
                    .default(|_| String::from("."))
                    .validate(|s| {
                        if s.len() < 10 {
                            Result::Ok(())
                        } else {
                            Result::Err(String::default())
                        }
                    }),
            )
            .add_value(CLIValue::new("target").default(|v| {
                let mut clone = v.get("source").unwrap().clone();
                clone.push_str("--");
                clone
            }));

        let p1 = app.consume(create_args!("")).unwrap();
        assert_eq!(p1.get_value("source").unwrap(), ".");
        assert_eq!(p1.get_value("target").unwrap(), ".--");

        let p2 = app.consume(create_args!("", "hello")).unwrap();
        assert_eq!(p2.get_value("source").unwrap(), "hello");
        assert_eq!(p2.get_value("target").unwrap(), "hello--");

        let p3 = app.consume(create_args!("", "hello", "world")).unwrap();
        assert_eq!(p3.get_value("source").unwrap(), "hello");
        assert_eq!(p3.get_value("target").unwrap(), "world");
    }

    #[test]
    fn fail_on_too_many_values() {
        let app = CLIApp::new()
            .name("hello-world")
            .add_value(CLIValue::new("source"))
            .add_value(CLIValue::new("target"));

        assert!(app.consume(create_args!("", "hello", "world")).is_ok());
        assert!(app
            .consume(create_args!("", "hello", "world", "bam"))
            .is_err());
    }
}
