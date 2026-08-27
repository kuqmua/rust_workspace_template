#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{}", constants_str::POSTGRESQL_IDEMPOTENCY_OPERATION_FAILED)]
pub struct SqlxPgTableIdempotencyError(#[source] pub(super) sqlx::Error);
impl to_err_string::domain_types::ToErrString for SqlxPgTableIdempotencyError {
    fn to_err_string(&self) -> to_err_string::domain_types::ErrorText {
        to_err_string::domain_types::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
    }
}
