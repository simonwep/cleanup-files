use std::collections::HashMap;

use flag::CLIFlag;
use result::CLIResult;
use value::CLIValue;

pub mod flag;
pub mod result;
pub mod value;

pub struct CLIApp {
    pub name: String,
    pub flags: Vec<CLIFlag>,
    pub values: Vec<CLIValue>
}

impl CLIApp {
    /// Creates a new cli-application
    pub fn new() -> Self {
        CLIApp {
            name: String::new(),
            flags: Vec::new(),
            values: Vec::new()
        }
    }

    /// Sets a name for this app
    pub fn set_name(mut self, name: &str) -> Self {
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
                panic!(format!(
                    "Flag with name \"{}\" is already defined.",
                    &new_flag.name
                ))
            }

            // Check if abbreviation is already in use
            for abbr in &flag.abbr {
                if new_flag.abbr.contains(abbr) {
                    panic!(format!(
                        "Flag with name \"{}\" uses \"{}\" which is already in use by \"{}\".",
                        &new_flag.name, &abbr, &flag.name
                    ))
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
            panic!(format!(
                "Value with name \"{}\" is already defined.",
                &val.name
            ))
        }

        self.values.push(val);
        self
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
                    Some(t) => t
                };

                // Copy name of flag
                let flag_name = target_flag.name.clone();

                // Parse value of flag if expected
                if target_flag.expects_value {
                    match iter
                        .next()
                        .or(target_flag.default.and_then(|def| Option::Some(def(&args))))
                    {
                        None => return Err(format!("Flag {} expects a value.", arg)),
                        Some(val) => {
                            // Save argument
                            args.insert(flag_name, val);
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
                        Ok(_) => ()
                    }
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
            if val.required {
                desc.push_str(&format!(" <{}?>", val.name.as_str()))
            } else {
                desc.push_str(&format!(" <{}>", val.name.as_str()))
            }
        }

        // If flag were set indicate them with that little thingy
        match self.flags.len() {
            0 => (),
            1 => desc.push_str(" [options]"),
            _ => desc.push_str(" [options...]")
        }

        desc
    }

    /// Prints a full help-text based on the previously defined attributes.
    pub fn print_help(&self) {
        // Print usage description
        println!("{}", self.usage_description());

        // Prepare flags for printing and aligning them
        let mut longest_flag: usize = 0;
        let mut flag_map: Vec<(String, String)> = Vec::new();
        for flag in &self.flags {
            let (usage, desc) = flag.to_string();

            // Update the maximum length of the command-syntax
            // This will be used to properly pad and align the commands later
            let len = usage.len();
            if len > longest_flag {
                longest_flag = len;
            }

            flag_map.push((desc, usage));
        }

        // Print flags
        for (name, flags) in flag_map {
            println!("  {: <width$}  {}", flags, name, width = longest_flag);
        }
    }
}
