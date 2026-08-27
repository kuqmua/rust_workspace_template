#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub(super) struct HttpOriginSchemeText(pub(super) String);

impl TryFrom<String> for HttpOriginSchemeText {
    type Error = super::AllowedOriginError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 16usize {
            Err(super::AllowedOriginError)
        } else {
            Ok(Self(value))
        }
    }
}
