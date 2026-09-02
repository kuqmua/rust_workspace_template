#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct PgRelationResourceIds(
    bounded_types::bounded_vec::BoundedVec<
        crate::pg_relation_resource_id::PgRelationResourceId,
        { constants_usize::ZERO },
        { crate::maximum_resource_count::MAXIMUM_RESOURCE_COUNT },
    >,
);

impl TryFrom<Vec<crate::pg_relation_resource_id::PgRelationResourceId>> for PgRelationResourceIds {
    type Error = crate::pg_relation_lock_error::PgRelationLockError;

    fn try_from(
        value: Vec<crate::pg_relation_resource_id::PgRelationResourceId>,
    ) -> Result<Self, Self::Error> {
        let mut resources = bounded_types::bounded_vec::BoundedVec::<
            crate::pg_relation_resource_id::PgRelationResourceId,
            { constants_usize::ZERO },
            { crate::maximum_resource_count::MAXIMUM_RESOURCE_COUNT },
        >::try_from(value)
        .map_err(|_error| crate::pg_relation_lock_error::PgRelationLockError::TooManyResources)?
        .into_inner();
        resources.sort();
        resources.dedup();
        bounded_types::bounded_vec::BoundedVec::try_from(resources)
            .map(Self)
            .map_err(|_error| crate::pg_relation_lock_error::PgRelationLockError::TooManyResources)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_resources_are_sorted_and_deduplicated_before_locking() {
        let resources = crate::pg_relation_resource_ids::PgRelationResourceIds::try_from(vec![
            crate::pg_relation_resource_id::PgRelationResourceId::from(2i64),
            crate::pg_relation_resource_id::PgRelationResourceId::from(constants_i64::ONE),
            crate::pg_relation_resource_id::PgRelationResourceId::from(2i64),
        ])
        .expect(constants_str::DIAGNOSTIC_A9CF9EA3);
        assert_eq!(
            resources.0.as_slice(),
            [
                crate::pg_relation_resource_id::PgRelationResourceId::from(constants_i64::ONE),
                crate::pg_relation_resource_id::PgRelationResourceId::from(2i64),
            ]
        );
    }
}
