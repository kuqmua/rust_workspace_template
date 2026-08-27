#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgRelationResourceIds(
    pub(super)  bounded_types::domain_types::vector::BoundedVec<
        crate::domain_types::PgRelationResourceId,
        { constants_usize::ZERO },
        { crate::domain_types::maximum_resource_count::MAXIMUM_RESOURCE_COUNT },
    >,
);

impl TryFrom<Vec<crate::domain_types::PgRelationResourceId>> for PgRelationResourceIds {
    type Error = crate::domain_types::PgRelationLockError;

    fn try_from(
        value: Vec<crate::domain_types::PgRelationResourceId>,
    ) -> Result<Self, Self::Error> {
        let mut resources = bounded_types::domain_types::vector::BoundedVec::<
            crate::domain_types::PgRelationResourceId,
            { constants_usize::ZERO },
            { crate::domain_types::maximum_resource_count::MAXIMUM_RESOURCE_COUNT },
        >::try_from(value)
        .map_err(|_error| crate::domain_types::PgRelationLockError::TooManyResources)?
        .into_inner();
        resources.sort_unstable();
        resources.dedup();
        bounded_types::domain_types::vector::BoundedVec::try_from(resources)
            .map(Self)
            .map_err(|_error| crate::domain_types::PgRelationLockError::TooManyResources)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn resources_are_sorted_and_deduplicated_before_locking() {
        let resources = super::PgRelationResourceIds::try_from(vec![
            crate::domain_types::PgRelationResourceId::from(2i64),
            crate::domain_types::PgRelationResourceId::from(constants_i64::ONE),
            crate::domain_types::PgRelationResourceId::from(2i64),
        ])
        .expect(
            "a9cf9ea3 resources_are_sorted_and_deduplicated_before_locking invariant must hold",
        );
        assert_eq!(
            resources.0.as_slice(),
            [
                crate::domain_types::PgRelationResourceId::from(constants_i64::ONE),
                crate::domain_types::PgRelationResourceId::from(2i64),
            ]
        );
    }
}
