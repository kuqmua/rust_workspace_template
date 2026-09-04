#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct PgTableIdempotencyResponseStatus(u16);
impl TryFrom<u16> for PgTableIdempotencyResponseStatus {
    type Error = crate::pg_table_idempotency_response_status_try_from_u16_error::PgTableIdempotencyResponseStatusTryFromU16Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            candidate if (100u16..1_000u16).contains(&candidate) => Ok(Self(candidate)),
            _ => Err(Self::Error::OutOfRange),
        }
    }
}
impl From<crate::pg_table_idempotency_known_response_status::PgTableIdempotencyKnownResponseStatus>
    for PgTableIdempotencyResponseStatus
{
    fn from(
        value: crate::pg_table_idempotency_known_response_status::PgTableIdempotencyKnownResponseStatus,
    ) -> Self {
        match value {
            crate::pg_table_idempotency_known_response_status::PgTableIdempotencyKnownResponseStatus::InternalServerError => Self(500u16),
        }
    }
}
