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
pub struct PgTableIdempotencyCleanupBatchSize(std::num::NonZeroI64);
impl TryFrom<i64> for PgTableIdempotencyCleanupBatchSize {
    type Error = crate::pg_table_idempotency_cleanup_value_try_from_i64_error::PgTableIdempotencyCleanupValueTryFromI64Error;

    fn try_from(i64: i64) -> Result<Self, Self::Error> {
        std::num::NonZeroI64::new(i64)
            .filter(|non_zero_value| non_zero_value.get().is_positive())
            .map(Self)
            .ok_or(crate::pg_table_idempotency_cleanup_value_try_from_i64_error::PgTableIdempotencyCleanupValueTryFromI64Error::NotPositive)
    }
}
