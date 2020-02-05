/// A CLIFlag represents a flag passed as cli-argument.
pub struct CLIFlag {
    pub name: String,
    pub description: String,
    pub value_description: String,
    pub expects_value: bool,
    pub validator: fn(&String) -> Result<(), String>,
    pub abbr: Vec<String>
}

/// Represents a cli-flag.
/// This is a builder class using mutable pointers of self and returning these.
/// This isn't best practice but it's the easiest way and I just want to use it as is.
impl CLIFlag {
    /// Creates a new CLIFlag.
    pub fn new(name: &str) -> CLIFlag {
        CLIFlag {
            name: name.to_owned(),
            description: String::from("Unknown"),
            value_description: String::from("value"),
            expects_value: false,
            abbr: Vec::new(),
            validator: |_| Ok(())
        }
    }

    /// Adds a new abbreviation.
    pub fn abbr(mut self, abbr: &str) -> CLIFlag {
        self.abbr.push(abbr.to_string());
        self
    }

    /// Makes the flag expecting a value passed to it.
    pub fn expects_value(mut self, value: bool) -> CLIFlag {
        self.expects_value = value;
        self
    }

    /// Updates the description of the value
    pub fn value_description(mut self, description: &str) -> CLIFlag {
        // Description can only be set on flags with value
        if !self.expects_value {
            panic!(format!(
                "Tried to set value-description on flag '{}' which does not expect a value.",
                self.name
            ))
        }

        self.value_description = description.to_string();
        self
    }

    /// Sets a validator for this flag
    pub fn validate(mut self, validator: fn(&String) -> Result<(), String>) -> CLIFlag {
        self.validator = validator;
        self
    }

    /// Sets a description
    pub fn description(mut self, description: &str) -> CLIFlag {
        self.description = description.to_string();
        self
    }

    /// Converts this flag to a readable string of how to use it.
    /// Returns a tuple with a usage-string and a clone of the description.
    pub fn to_string(&self) -> (String, String) {
        let mut usage: String = self.abbr.join(", ");

        // Add description of expected value if provided
        if self.expects_value {
            usage.push_str(&format!(" <{}>", self.value_description));
        }

        (usage, self.description.clone())
    }

    /// Checks whenever this flag contains a specific abbreviation.
    pub fn has_abbr(&self, other: &String) -> bool {
        self.abbr.contains(other)
    }
}
