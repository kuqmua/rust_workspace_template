#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::domain_types::{SNAKE_IDENT_MAX_LEN, SnakeIdentifierifierLen};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct SnakeIdentifierifierTryFromStringError(pub(super) SnakeIdentifierifierLen);
impl From<SnakeIdentifierifierLen> for SnakeIdentifierifierTryFromStringError {
    fn from(value: SnakeIdentifierifierLen) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for SnakeIdentifierifierTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "snake identifier length {} exceeds maximum {SNAKE_IDENT_MAX_LEN}",
            self.0.0
        )
    }
}
