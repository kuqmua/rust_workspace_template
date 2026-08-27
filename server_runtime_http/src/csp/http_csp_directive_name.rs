#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpCspDirectiveName(pub(super) String);

impl TryFrom<String> for HttpCspDirectiveName {
    type Error = super::HttpCspTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(super::HttpCspTokenError::Empty);
        }
        if value.len() > constants_usize::VALUE_64 {
            return Err(super::HttpCspTokenError::TooLong);
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
        {
            return Err(super::HttpCspTokenError::InvalidCharacter);
        }
        Ok(Self(value))
    }
}
