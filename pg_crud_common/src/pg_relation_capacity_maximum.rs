#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRelationCapacityMaximum(
    pub(super) crate::domain_types::pg_relation_capacity_maximum_non_zero_u64::PgRelationCapacityMaximumNonZeroU64,
);

impl TryFrom<u64> for PgRelationCapacityMaximum {
    type Error = crate::domain_types::PgRelationCapacityError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(crate::domain_types::PgRelationCapacityError::ZeroMaximum)
    }
}

impl From<std::num::NonZeroU64> for PgRelationCapacityMaximum {
    fn from(value: std::num::NonZeroU64) -> Self {
        Self(
            crate::domain_types::pg_relation_capacity_maximum_non_zero_u64::PgRelationCapacityMaximumNonZeroU64::from(value),
        )
    }
}
