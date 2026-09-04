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
    proc_macro_newtype_get_inner::GetInner,
)]
pub struct PgTableIdempotencyCleanupRetentionSeconds(i64);
impl TryFrom<i64> for PgTableIdempotencyCleanupRetentionSeconds {
    type Error = crate::pg_table_idempotency_cleanup_value_try_from_i64_error::PgTableIdempotencyCleanupValueTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < constants_i64::ZERO {
            Err(Self::Error::Negative)
        } else {
            Ok(Self(value))
        }
    }
}
