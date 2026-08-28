#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, newtype::TryFrom,
)]
#[try_from(
    error = PgTableIdempotencyCleanupValueTryFromI64Error,
    validator = |value: &i64| {
        if *value < constants_i64::ZERO { Err(PgTableIdempotencyCleanupValueTryFromI64Error::Negative) } else { Ok(()) }
    }
)]
pub struct PgTableIdempotencyCleanupRetentionSeconds(pub(super) i64);
