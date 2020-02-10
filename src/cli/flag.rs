use std::collections::HashMap;

/// A CLIFlag represents a flag passed as cli-argument.
pub struct CLIFlag {
    pub name: String,
    pub default: Option<fn(&HashMap<String, String>) -> String>,
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
    pub fn new(name: &str) -> Self {
        CLIFlag {
            name: name.to_owned(),
            default: Option::None,
            description: String::default(),
            value_description: String::default(),
            expects_value: false,
            abbr: Vec::new(),
            validator: |_| Ok(())
        }
    }

    /// Adds a new abbreviation.
    pub fn abbr(mut self, abbr: &str) -> Self {
        self.abbr.push(abbr.to_string());
        self
    }

    /// Makes the flag expecting a value passed to it.
    pub fn expects_value(mut self, value: bool) -> Self {
        self.expects_value = value;
        self
    }

    /// Sets a default value
    pub fn default(mut self, default: fn(&HashMap<String, String>) -> String) -> Self {
        self.default = Option::Some(default);
        self.expects_value = true;
        self
    }

    pub fn resolve_default(&self, map: &HashMap<String, String>) -> Option<String> {
        self.default.and_then(|func| Option::Some(func(map)))
    }

    /// Updates the description of the value
    pub fn value_description(mut self, description: &str) -> Self {
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
    pub fn validate(mut self, validator: fn(&String) -> Result<(), String>) -> Self {
        self.validator = validator;
        self
    }

    /// Sets a description
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Converts this flag to a readable string of how to use it.
    /// Returns a tuple with a usage-string and a clone of the description.
    pub fn stringify(&self) -> (String, String) {
        let mut usage: String = self.abbr.join(", ");

        // Add description of expected value if provided
        if self.expects_value {
            usage.push_str(&format!(" <{}>", self.value_description));
        }

        (usage, self.description.clone())
    }

    /// Checks whenever this flag contains a specific abbreviation.
    pub fn has_abbr(&self, other: &str) -> bool {
        self.abbr.contains(&other.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::cli::flag::*;

    #[test]
    fn test_create() {
        let flag = CLIFlag::new("Hello")
            .description("Hello World")
            .abbr("-a")
            .abbr("--abbr")
            .abbr("--abbr-ex");

        assert_eq!(flag.name, "Hello");
        assert_eq!(flag.description, "Hello World");
        assert_eq!(flag.abbr, vec!["-a", "--abbr", "--abbr-ex"]);
    }

    #[test]
    fn test_create_with_value() {
        let flag = CLIFlag::new("Hello")
            .expects_value(true)
            .value_description("My Value");

        assert_eq!(flag.value_description, "My Value");
    }

    #[test]
    #[should_panic]
    fn test_with_invalid_value_description_call() {
        CLIFlag::new("Hello").value_description("My Value");
    }

    #[test]
    fn test_to_string() {
        let (usage, desc) = CLIFlag::new("Hello")
            .expects_value(true)
            .value_description("hello")
            .description("Hello World")
            .abbr("-a")
            .abbr("--abbr")
            .stringify();

        assert_eq!(desc, "Hello World");
        assert_eq!(usage, "-a, --abbr <hello>")
    }

    #[test]
    fn test_has_abbr() {
        let flag = CLIFlag::new("Hello")
            .abbr("-a")
            .abbr("--abbr")
            .abbr("--woo")
            .abbr("--woo-baz");

        assert!(flag.has_abbr("-a"));
        assert!(flag.has_abbr("--abbr"));
        assert!(flag.has_abbr("--woo"));
        assert!(flag.has_abbr("--woo-baz"));
        assert!(!flag.has_abbr("--woo-bum"));
        assert!(!flag.has_abbr("--foo"));
        assert!(!flag.has_abbr("s"));
    }
}
