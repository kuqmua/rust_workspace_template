#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::domain_types::FIRST_IDENT_MAX_LEN;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstIdentifierifierTryFromStringError(pub(super) usize);
impl From<usize> for FirstIdentifierifierTryFromStringError {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for FirstIdentifierifierTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "first identifier length {} exceeds maximum {FIRST_IDENT_MAX_LEN}",
            self.0
        )
    }
}
