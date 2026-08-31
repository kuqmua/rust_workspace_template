#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct PgRelationCapacityMaximum(std::num::NonZeroU64);

impl TryFrom<u64> for PgRelationCapacityMaximum {
    type Error = crate::pg_relation_capacity_error::PgRelationCapacityError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(crate::pg_relation_capacity_error::PgRelationCapacityError::ZeroMaximum)
    }
}
