fn catalog_snapshot(
    db_object_kind: crate::db_object_kind::DbObjectKind,
) -> crate::db_catalog_snapshot::DbCatalogSnapshot {
    crate::db_catalog_snapshot::DbCatalogSnapshot::new(vec![object_snapshot(db_object_kind)].into())
}

fn object_snapshot(
    db_object_kind: crate::db_object_kind::DbObjectKind,
) -> crate::db_object_snapshot::DbObjectSnapshot {
    crate::db_object_snapshot::DbObjectSnapshot::new(
        crate::db_schema_text::DbSchemaText::try_from(String::from(
            constants_str::TEST_DB_OBJECT_NAME,
        ))
        .expect(constants_str::DIAGNOSTIC_E84FED1B),
        db_object_kind,
        crate::db_schema_text::DbSchemaText::try_from(String::from(
            constants_str::TEST_DB_OBJECT_DEFINITION,
        ))
        .expect(constants_str::DIAGNOSTIC_A7950FF0),
    )
}

fn snapshot(bool: bool) -> crate::db_table_snapshot::DbTableSnapshot {
    crate::db_table_snapshot::DbTableSnapshot::new(
        vec![column_snapshot(bool)].into(),
        vec![crate::db_object_snapshot::DbObjectSnapshot::new(
            crate::db_schema_text::DbSchemaText::try_from(String::from(
                constants_str::TEST_DB_CONSTRAINT_NAME,
            ))
            .expect(constants_str::VALUE_61F95647),
            crate::db_object_kind::DbObjectKind::PrimaryKey,
            crate::db_schema_text::DbSchemaText::try_from(String::from(
                constants_str::TEST_DB_CONSTRAINT_DEFINITION,
            ))
            .expect(constants_str::VALUE_A4B28D38),
        )]
        .into(),
    )
}

#[test]
fn test_ordering_does_not_affect_snapshot_and_differences_are_reported() {
    assert!(matches!(
        crate::validate_postgres_table_schema::validate_postgres_table_schema(
            snapshot(false),
            snapshot(false)
        ),
        Ok(())
    ));
    assert!(matches!(
        crate::validate_postgres_table_schema::validate_postgres_table_schema(
            snapshot(false),
            snapshot(true)
        ),
        Err(crate::db_schema_conformance_error::DbSchemaConformanceError::Mismatch { .. })
    ));
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator keeps the all-kinds assertion compact under the workspace no-for-loop policy"
)]
fn test_every_catalog_object_kind_difference_is_reported() {
    let kinds = [
        crate::db_object_kind::DbObjectKind::Check,
        crate::db_object_kind::DbObjectKind::Default,
        crate::db_object_kind::DbObjectKind::Extension,
        crate::db_object_kind::DbObjectKind::ForeignKey,
        crate::db_object_kind::DbObjectKind::Function,
        crate::db_object_kind::DbObjectKind::Index,
        crate::db_object_kind::DbObjectKind::PrimaryKey,
        crate::db_object_kind::DbObjectKind::Trigger,
        crate::db_object_kind::DbObjectKind::Unique,
        crate::db_object_kind::DbObjectKind::View,
    ];
    kinds.into_iter().for_each(|kind| {
        let result = crate::validate_postgres_catalog::validate_postgres_catalog(
            catalog_snapshot(crate::db_object_kind::DbObjectKind::Function),
            catalog_snapshot(kind),
        );
        if kind == crate::db_object_kind::DbObjectKind::Function {
            assert!(matches!(result, Ok(())));
        } else {
            assert!(matches!(
                result,
                Err(crate::db_schema_conformance_error::DbSchemaConformanceError::CatalogMismatch { .. })
            ));
        }
    });
}

fn column_snapshot(bool: bool) -> crate::db_column_snapshot::DbColumnSnapshot {
    crate::db_column_snapshot::DbColumnSnapshot::new(
        crate::db_schema_text::DbSchemaText::try_from(String::from(
            constants_str::TEST_DB_COLUMN_ID,
        ))
        .expect(constants_str::DIAGNOSTIC_11F0D7F5),
        crate::db_schema_text::DbSchemaText::try_from(String::from(
            constants_str::TEST_DB_DATA_TYPE_UUID,
        ))
        .expect(constants_str::DIAGNOSTIC_9CB64C93),
        bool.into(),
        None,
    )
}

#[test]
fn test_multiple_table_columns_and_objects_are_order_independent() {
    let expected = crate::db_table_snapshot::DbTableSnapshot::new(
        vec![column_snapshot(false), column_snapshot(true)].into(),
        vec![
            object_snapshot(crate::db_object_kind::DbObjectKind::PrimaryKey),
            object_snapshot(crate::db_object_kind::DbObjectKind::Function),
        ]
        .into(),
    );
    let observed = crate::db_table_snapshot::DbTableSnapshot::new(
        vec![column_snapshot(true), column_snapshot(false)].into(),
        vec![
            object_snapshot(crate::db_object_kind::DbObjectKind::Function),
            object_snapshot(crate::db_object_kind::DbObjectKind::PrimaryKey),
        ]
        .into(),
    );
    assert_eq!(expected, observed);
    assert!(matches!(
        crate::validate_postgres_table_schema::validate_postgres_table_schema(expected, observed),
        Ok(())
    ));
}

#[test]
fn test_catalog_ordering_preserves_duplicate_objects() {
    let primary_key = object_snapshot(crate::db_object_kind::DbObjectKind::PrimaryKey);
    let function = object_snapshot(crate::db_object_kind::DbObjectKind::Function);
    let expected = crate::db_catalog_snapshot::DbCatalogSnapshot::new(
        vec![primary_key.clone(), function.clone(), primary_key.clone()].into(),
    );
    let reordered = crate::db_catalog_snapshot::DbCatalogSnapshot::new(
        vec![function.clone(), primary_key.clone(), primary_key.clone()].into(),
    );
    assert!(matches!(
        crate::validate_postgres_catalog::validate_postgres_catalog(expected.clone(), reordered),
        Ok(())
    ));
    let observed =
        crate::db_catalog_snapshot::DbCatalogSnapshot::new(vec![primary_key, function].into());
    let result = crate::validate_postgres_catalog::validate_postgres_catalog(
        expected.clone(),
        observed.clone(),
    );
    assert!(
        matches!(result, Err(crate::db_schema_conformance_error::DbSchemaConformanceError::CatalogMismatch { expected: actual_expected, observed: actual_observed }) if actual_expected == expected && actual_observed == observed)
    );
}

#[test]
fn test_table_mismatch_preserves_both_snapshots() {
    let expected = snapshot(false);
    let observed = snapshot(true);
    let result = crate::validate_postgres_table_schema::validate_postgres_table_schema(
        expected.clone(),
        observed.clone(),
    );
    assert!(
        matches!(result, Err(crate::db_schema_conformance_error::DbSchemaConformanceError::Mismatch { expected: actual_expected, observed: actual_observed }) if actual_expected == expected && actual_observed == observed)
    );
}

#[test]
fn test_table_schema_comparison_preserves_duplicate_column_and_object_counts() {
    let table_snapshot = |column_count, object_count| {
        crate::db_table_snapshot::DbTableSnapshot::new(
            vec![column_snapshot(false); column_count].into(),
            vec![object_snapshot(crate::db_object_kind::DbObjectKind::PrimaryKey); object_count]
                .into(),
        )
    };
    assert!([
        ((2usize, 2usize), (2usize, 2usize), true),
        ((2usize, 2usize), (1usize, 2usize), false),
        ((2usize, 2usize), (2usize, 1usize), false),
        ((0usize, 0usize), (0usize, 0usize), true),
        ((0usize, 0usize), (1usize, 0usize), false),
        ((0usize, 0usize), (0usize, 1usize), false),
        ((2usize, 2usize), (0usize, 0usize), false),
        ((0usize, 0usize), (2usize, 2usize), false),
    ].into_iter().all(|((expected_columns, expected_objects), (observed_columns, observed_objects), should_match)| {
        let expected = table_snapshot(expected_columns, expected_objects);
        let observed = table_snapshot(observed_columns, observed_objects);
        let result = crate::validate_postgres_table_schema::validate_postgres_table_schema(expected.clone(), observed.clone());
        if should_match {
            matches!(result, Ok(()))
        } else {
            matches!(result, Err(crate::db_schema_conformance_error::DbSchemaConformanceError::Mismatch { expected: actual_expected, observed: actual_observed }) if actual_expected == expected && actual_observed == observed)
        }
    }));
}
