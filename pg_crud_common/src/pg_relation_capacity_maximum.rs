#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_relation_capacity_requires_nonzero_maximum() {
        assert_eq!(
            super::PgRelationCapacityMaximum::try_from(0u64),
            Err(crate::pg_relation_capacity_error::PgRelationCapacityError::ZeroMaximum)
        );
        assert!(
            super::PgRelationCapacityMaximum::try_from(1u64)
                .is_ok_and(|maximum| maximum.get_inner().get() == 1u64)
        );
        assert!(
            super::PgRelationCapacityMaximum::try_from(u64::MAX)
                .is_ok_and(|maximum| maximum.get_inner().get() == u64::MAX)
        );
    }
}
