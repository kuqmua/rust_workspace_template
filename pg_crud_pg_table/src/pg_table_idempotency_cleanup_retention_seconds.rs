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
    proc_macro_newtype::GetInner,
    proc_macro_newtype::TryFrom,
)]
#[try_from(
    error = crate::pg_table_idempotency_cleanup_value_try_from_i64_error::PgTableIdempotencyCleanupValueTryFromI64Error,
    validator = |value: &i64| {
        if *value < constants_i64::ZERO { Err(crate::pg_table_idempotency_cleanup_value_try_from_i64_error::PgTableIdempotencyCleanupValueTryFromI64Error::Negative) } else { Ok(()) }
    }
)]
pub struct PgTableIdempotencyCleanupRetentionSeconds(i64);
