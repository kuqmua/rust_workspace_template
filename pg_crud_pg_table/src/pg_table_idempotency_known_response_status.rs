#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgTableIdempotencyKnownResponseStatus {
    InternalServerError,
}
impl From<PgTableIdempotencyKnownResponseStatus>
    for crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus
{
    fn from(value: PgTableIdempotencyKnownResponseStatus) -> Self {
        match value {
            PgTableIdempotencyKnownResponseStatus::InternalServerError => Self(500u16),
        }
    }
}
impl crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus {
    #[must_use]
    pub fn internal_server_error() -> Self {
        Self::from(PgTableIdempotencyKnownResponseStatus::InternalServerError)
    }
}
