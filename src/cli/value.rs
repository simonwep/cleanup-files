/// A CLIValue represents a stand-alone value passed without and flag prepended
pub struct CLIValue {
    pub name: String,
    pub default: String,
    pub has_default: bool,
    pub required: bool,
    pub description: String,
    pub validator: fn(&String) -> Result<(), String>
}

impl CLIValue {
    /// Creates a new CLIValue
    pub fn new(name: &str) -> CLIValue {
        CLIValue {
            name: name.to_owned(),
            default: String::default(),
            has_default: false,
            required: true,
            description: String::from("Unknown"),
            validator: |_| Ok(())
        }
    }

    /// Sets a default value
    pub fn default(mut self, default: &str) -> CLIValue {
        self.has_default = true;
        self.default = default.to_owned();
        self
    }

    /// Marks this value as optional or required
    pub fn required(mut self, required: bool) -> CLIValue {
        self.required = required;
        self
    }

    /// Sets a validator for this value
    pub fn validate(mut self, validator: fn(&String) -> Result<(), String>) -> CLIValue {
        self.validator = validator;
        self
    }

    /// Sets a description
    pub fn description(mut self, description: &str) -> CLIValue {
        self.description = description.to_string();
        self
    }
}
