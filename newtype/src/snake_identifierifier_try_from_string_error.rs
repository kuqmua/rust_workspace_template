#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct SnakeIdentifierifierTryFromStringError(
    pub(super) crate::snake_identifierifier_len::SnakeIdentifierifierLen,
);
impl From<crate::snake_identifierifier_len::SnakeIdentifierifierLen>
    for SnakeIdentifierifierTryFromStringError
{
    fn from(value: crate::snake_identifierifier_len::SnakeIdentifierifierLen) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for SnakeIdentifierifierTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "snake identifier length {} exceeds maximum {}",
            self.0.0,
            crate::snake_ident_max_len::SNAKE_IDENT_MAX_LEN
        )
    }
}
