#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgTableIdempotencyKnownResponseStatus {
    InternalServerError,
}
impl From<PgTableIdempotencyKnownResponseStatus> for PgTableIdempotencyResponseStatus {
    fn from(value: PgTableIdempotencyKnownResponseStatus) -> Self {
        match value {
            PgTableIdempotencyKnownResponseStatus::InternalServerError => Self(500u16),
        }
    }
}
impl PgTableIdempotencyResponseStatus {
    #[must_use]
    pub fn internal_server_error() -> Self {
        Self::from(PgTableIdempotencyKnownResponseStatus::InternalServerError)
    }
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    pub(super) fn validate(
        value: &u16,
    ) -> Result<(), PgTableIdempotencyResponseStatusTryFromU16Error> {
        if (100u16..1_000u16).contains(value) {
            Ok(())
        } else {
            Err(PgTableIdempotencyResponseStatusTryFromU16Error)
        }
    }
}
