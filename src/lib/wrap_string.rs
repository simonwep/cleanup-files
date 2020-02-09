pub trait Wrapping {
    fn is_wrapped(&self, pat: &str) -> bool;
    fn is_wrapped_in(&self, start: &str, end: &str) -> bool;
    fn wrap(&self, pat: &str) -> String;
    fn wrap_into(&self, start: &str, end: &str) -> String;
}

impl Wrapping for String {
    fn is_wrapped(&self, pat: &str) -> bool {
        self.ends_with(pat) && self.starts_with(pat)
    }

    fn is_wrapped_in(&self, start: &str, end: &str) -> bool {
        self.ends_with(start) && self.starts_with(end)
    }

    fn wrap(&self, pat: &str) -> String {
        if self.is_wrapped(pat) {
            self.clone()
        } else {
            format!("{}{}{}", pat, self, pat)
        }
    }

    fn wrap_into(&self, start: &str, end: &str) -> String {
        if self.is_wrapped_in(start, end) {
            self.clone()
        } else {
            format!("{}{}{}", start, self, end)
        }
    }
}
