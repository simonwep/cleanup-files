use std::collections::HashMap;

/// A CLIValue represents a stand-alone value passed without and flag prepended
pub struct CLIValue {
    pub name: String,
    pub default: Option<fn(&HashMap<String, String>) -> String>,
    pub required: bool,
    pub description: String,
    pub validator: Option<fn(&String) -> Result<(), String>>
}

impl CLIValue {
    /// Creates a new CLIValue
    pub fn new(name: &str) -> Self {
        CLIValue {
            name: name.to_owned(),
            default: Option::None,
            required: true,
            description: String::from("Unknown"),
            validator: Option::None
        }
    }

    /// Sets a default value
    pub fn default(mut self, default: fn(&HashMap<String, String>) -> String) -> Self {
        self.default = Option::Some(default);
        self
    }

    /// Marks this value as optional or required
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Sets a validator for this value
    pub fn validate(mut self, validator: fn(&String) -> Result<(), String>) -> Self {
        self.validator = Option::Some(validator);
        self
    }

    /// Sets a description
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }
}
