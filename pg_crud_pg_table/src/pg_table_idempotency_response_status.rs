#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = crate::pg_table_idempotency_response_status_try_from_u16_error::PgTableIdempotencyResponseStatusTryFromU16Error,
    validator = |value: &u16| {
        if (100u16..1_000u16).contains(value) { Ok(()) } else { Err(crate::pg_table_idempotency_response_status_try_from_u16_error::PgTableIdempotencyResponseStatusTryFromU16Error) }
    }
)]
pub struct PgTableIdempotencyResponseStatus(pub(super) u16);
