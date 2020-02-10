pub trait Wrapping {
    fn is_wrapped_in(&self, start: &str, end: &str) -> bool;
    fn is_wrapped(&self, pat: &str) -> bool;
    fn wrap(&self, pat: &str) -> String;
    fn wrap_into(&self, start: &str, end: &str) -> String;
}

impl Wrapping for String {
    fn is_wrapped_in(&self, start: &str, end: &str) -> bool {
        self.starts_with(start) && self.ends_with(end)
    }

    fn is_wrapped(&self, pat: &str) -> bool {
        self.is_wrapped_in(pat, pat)
    }

    fn wrap(&self, pat: &str) -> String {
        if self.is_wrapped(pat) {
            self.clone()
        } else {
            vec![pat, self, pat].concat()
        }
    }

    fn wrap_into(&self, start: &str, end: &str) -> String {
        if self.is_wrapped_in(start, end) {
            self.clone()
        } else {
            vec![start, self, end].concat()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lib::*;

    #[test]
    fn wrap_string() {
        assert_eq!(String::from("hello").wrap("'"), "'hello'");
        assert_eq!(String::from("'hello'").wrap("'"), "'hello'");
        assert_eq!(String::from("hello").wrap_into("<", ">"), "<hello>");
    }

    #[test]
    fn prevent_double_wrapping() {
        assert_eq!(String::from("'hello'").wrap("'"), "'hello'");
        assert_eq!(String::from("<hello>").wrap_into("<", ">"), "<hello>");
    }

    #[test]
    fn check_if_wrapped() {
        assert!(String::from("<hello>").is_wrapped_in("<", ">"));
        assert!(String::from("#hello#").is_wrapped("#"));
    }
}
