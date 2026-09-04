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
    proc_macro_newtype_try_from::TryFrom,
)]
#[try_from(
    error = crate::pg_table_idempotency_response_status_try_from_u16_error::PgTableIdempotencyResponseStatusTryFromU16Error,
    validator = |value: &u16| {
        if (100u16..1_000u16).contains(value) { Ok(()) } else { Err(crate::pg_table_idempotency_response_status_try_from_u16_error::PgTableIdempotencyResponseStatusTryFromU16Error::OutOfRange) }
    }
)]
pub struct PgTableIdempotencyResponseStatus(u16);
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
