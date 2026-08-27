#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpCspDirectiveValue(pub(super) String);

impl TryFrom<String> for HttpCspDirectiveValue {
    type Error = super::HttpCspTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(super::HttpCspTokenError::Empty);
        }
        if value.len() > constants_usize::VALUE_1_024 {
            return Err(super::HttpCspTokenError::TooLong);
        }
        if value
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() || byte == b';')
        {
            return Err(super::HttpCspTokenError::InvalidCharacter);
        }
        Ok(Self(value))
    }
}
