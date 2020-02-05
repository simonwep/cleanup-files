use std::collections::HashMap;

pub struct CLIResult {
    values: HashMap<String, String>,
    args: HashMap<String, String>,
    flags: Vec<String>
}

impl CLIResult {
    pub fn from(
        values: HashMap<String, String>,
        args: HashMap<String, String>,
        flags: Vec<String>
    ) -> CLIResult {
        CLIResult {
            values,
            args,
            flags
        }
    }

    /// Checks whenever a flag is set.
    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.contains(&name.to_string())
    }

    /// Checks whenever a argument exist.
    pub fn has_arg(&self, name: &str) -> bool {
        self.args.contains_key(name)
    }

    /// Resolves an argument.
    pub fn get_arg(&self, name: &str) -> Option<&String> {
        self.args.get(name)
    }

    /// Checks whenever a value exist.
    pub fn has_value(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// Resolves a value.
    pub fn get_value(&self, name: &str) -> Option<&String> {
        self.values.get(name)
    }
}
