#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyCleanupBatchSize(pub(super) std::num::NonZeroI64);
impl TryFrom<i64> for PgTableIdempotencyCleanupBatchSize {
    type Error = PgTableIdempotencyCleanupValueTryFromI64Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        std::num::NonZeroI64::new(value)
            .filter(|non_zero_value| non_zero_value.get().is_positive())
            .map(Self)
            .ok_or(PgTableIdempotencyCleanupValueTryFromI64Error::NotPositive)
    }
}
