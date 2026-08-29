#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRelationCapacityMaximum(pub(super) std::num::NonZeroU64);

impl TryFrom<u64> for PgRelationCapacityMaximum {
    type Error = crate::pg_relation_capacity_error::PgRelationCapacityError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(crate::pg_relation_capacity_error::PgRelationCapacityError::ZeroMaximum)
    }
}

impl From<std::num::NonZeroU64> for PgRelationCapacityMaximum {
    fn from(value: std::num::NonZeroU64) -> Self {
        Self(value)
    }
}
