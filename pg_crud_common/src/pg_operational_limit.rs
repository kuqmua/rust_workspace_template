#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd,
)]
pub struct PgOperationalLimit(
    pub(super) crate::domain_types::pg_operational_limit_non_zero_u64::PgOperationalLimitNonZeroU64,
);

impl TryFrom<u64> for PgOperationalLimit {
    type Error = crate::domain_types::PgOperationalLimitError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(crate::domain_types::PgOperationalLimitError::ZeroLimit)
    }
}

impl From<std::num::NonZeroU64> for PgOperationalLimit {
    fn from(value: std::num::NonZeroU64) -> Self {
        Self(
            crate::domain_types::pg_operational_limit_non_zero_u64::PgOperationalLimitNonZeroU64::from(value),
        )
    }
}
