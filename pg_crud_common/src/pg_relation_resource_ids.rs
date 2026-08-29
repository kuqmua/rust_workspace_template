#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgRelationResourceIds(
    pub(super)  bounded_types::bounded_vec::BoundedVec<
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
        resources.sort_unstable();
        resources.dedup();
        bounded_types::bounded_vec::BoundedVec::try_from(resources)
            .map(Self)
            .map_err(|_error| crate::pg_relation_lock_error::PgRelationLockError::TooManyResources)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn resources_are_sorted_and_deduplicated_before_locking() {
        let resources = crate::pg_relation_resource_ids::PgRelationResourceIds::try_from(vec![
            crate::pg_relation_resource_id::PgRelationResourceId::from(2i64),
            crate::pg_relation_resource_id::PgRelationResourceId::from(constants_i64::ONE),
            crate::pg_relation_resource_id::PgRelationResourceId::from(2i64),
        ])
        .expect(
            "a9cf9ea3 resources_are_sorted_and_deduplicated_before_locking invariant must hold",
        );
        assert_eq!(
            resources.0.as_slice(),
            [
                crate::pg_relation_resource_id::PgRelationResourceId::from(constants_i64::ONE),
                crate::pg_relation_resource_id::PgRelationResourceId::from(2i64),
            ]
        );
    }
}
