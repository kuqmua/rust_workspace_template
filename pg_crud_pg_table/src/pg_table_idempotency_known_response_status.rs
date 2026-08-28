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
}
