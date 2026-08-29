#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{}", constants_str::catalog::POSTGRESQL_IDEMPOTENCY_OPERATION_FAILED)]
pub struct SqlxPgTableIdempotencyError(#[source] pub(super) sqlx::Error);
impl to_err_string::to_err_string::ToErrString for SqlxPgTableIdempotencyError {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
