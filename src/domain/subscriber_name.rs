use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberName {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty_or_only_whitespace = s.trim().is_empty();
        let forbidden_symbols = ['(', ')', '/', '\\', '"', '<', '>', '{', '}'];
        let is_too_long = s.graphemes(true).count() > 256;
        let contains_forbidden_characters = s.chars().any(|g| forbidden_symbols.contains(&g));

        if is_empty_or_only_whitespace || is_too_long || contains_forbidden_characters {
            return Err(format!("{} is not a valid subscriber name.", s));
        }
        Ok(Self(s))
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberName;
    use claim::{assert_err, assert_ok};
    #[test]
    fn a_256_graphem_long_name_is_valid() {
        let name = "a̐".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_forbidden_chars_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }
}
