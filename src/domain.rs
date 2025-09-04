use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {
        let is_empty_or_only_whitespace = s.trim().is_empty();
        let forbidden_symbols = ['(', ')', '/', '\\', '"', '<', '>', '{', '}'];
        let is_too_long = s.graphemes(true).count() > 255;
        let contains_forbidden_characters = s.chars().any(|g| forbidden_symbols.contains(&g));

        if is_empty_or_only_whitespace || is_too_long || contains_forbidden_characters {
            todo!()
        }
        SubscriberName(s)
    }
}
