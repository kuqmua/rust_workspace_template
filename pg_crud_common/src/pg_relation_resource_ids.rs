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

    #[test]
    fn test_relation_resource_input_limits_precede_deduplication() {
        let resource = crate::pg_relation_resource_id::PgRelationResourceId::from(i64::MAX);
        [
            0usize,
            1usize,
            crate::maximum_resource_count::MAXIMUM_RESOURCE_COUNT,
        ]
        .into_iter()
        .fold((), |(), count| {
            let repeated_resources = vec![resource; count];
            let resources = crate::pg_relation_resource_ids::PgRelationResourceIds::try_from(
                repeated_resources,
            )
            .unwrap_or_else(|error| std::panic::panic_any(error));
            assert_eq!(
                resources.get_inner().as_slice(),
                if count == 0usize {
                    &[][..]
                } else {
                    std::slice::from_ref(&resource)
                }
            );
        });
        let oversized_resources = vec![
            resource;
            crate::maximum_resource_count::MAXIMUM_RESOURCE_COUNT
                .saturating_add(1usize)
        ];
        assert_eq!(
            crate::pg_relation_resource_ids::PgRelationResourceIds::try_from(oversized_resources),
            Err(crate::pg_relation_lock_error::PgRelationLockError::TooManyResources)
        );
    }

    #[test]
    fn test_relation_resources_preserve_full_capacity_order_and_signed_extremes() {
        let limit = crate::maximum_resource_count::MAXIMUM_RESOURCE_COUNT;
        let values = (0usize..limit)
            .rev()
            .map(|value| {
                i64::try_from(value).map(crate::pg_relation_resource_id::PgRelationResourceId::from)
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|error| std::panic::panic_any(error));
        let resources = crate::pg_relation_resource_ids::PgRelationResourceIds::try_from(values)
            .unwrap_or_else(|error| std::panic::panic_any(error));
        let signed_limit =
            i64::try_from(limit).unwrap_or_else(|error| std::panic::panic_any(error));
        assert_eq!(resources.get_inner().as_slice().len(), limit);
        assert!(
            resources
                .get_inner()
                .as_slice()
                .iter()
                .map(|resource| *resource.get_inner())
                .eq(0i64..signed_limit)
        );
        let extremes = crate::pg_relation_resource_ids::PgRelationResourceIds::try_from(
            [i64::MAX, i64::MIN, 0i64, i64::MAX, i64::MIN]
                .into_iter()
                .map(crate::pg_relation_resource_id::PgRelationResourceId::from)
                .collect::<Vec<_>>(),
        )
        .unwrap_or_else(|error| std::panic::panic_any(error));
        assert!(
            extremes
                .get_inner()
                .as_slice()
                .iter()
                .map(|resource| *resource.get_inner())
                .eq([i64::MIN, 0i64, i64::MAX])
        );
    }
}
