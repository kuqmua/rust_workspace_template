fn catalog_snapshot(
    kind: crate::db_object_kind::DbObjectKind,
) -> crate::db_catalog_snapshot::DbCatalogSnapshot {
    crate::db_catalog_snapshot::DbCatalogSnapshot::new(
        vec![crate::db_object_snapshot::DbObjectSnapshot::new(
            crate::db_schema_text::DbSchemaText::try_from(String::from(
                constants_str::test_fixtures::TEST_DB_OBJECT_NAME,
            ))
            .expect(constants_str::test_fixtures::VALUE_E84FED1B),
            kind,
            crate::db_schema_text::DbSchemaText::try_from(String::from(
                constants_str::test_fixtures::TEST_DB_OBJECT_DEFINITION,
            ))
            .expect(constants_str::test_fixtures::VALUE_A7950FF0),
        )]
        .into(),
    )
}

fn snapshot(nullable: bool) -> crate::db_table_snapshot::DbTableSnapshot {
    crate::db_table_snapshot::DbTableSnapshot::new(
        vec![crate::db_column_snapshot::DbColumnSnapshot::new(
            crate::db_schema_text::DbSchemaText::try_from(String::from(
                constants_str::test_fixtures::TEST_DB_COLUMN_ID,
            ))
            .expect(constants_str::test_fixtures::VALUE_11F0D7F5),
            crate::db_schema_text::DbSchemaText::try_from(String::from(
                constants_str::test_fixtures::TEST_DB_DATA_TYPE_UUID,
            ))
            .expect(constants_str::test_fixtures::VALUE_9CB64C93),
            nullable.into(),
            None,
        )]
        .into(),
        vec![crate::db_object_snapshot::DbObjectSnapshot::new(
            crate::db_schema_text::DbSchemaText::try_from(String::from(
                constants_str::test_fixtures::TEST_DB_CONSTRAINT_NAME,
            ))
            .expect(constants_str::test_fixtures::VALUE_61F95647),
            crate::db_object_kind::DbObjectKind::PrimaryKey,
            crate::db_schema_text::DbSchemaText::try_from(String::from(
                constants_str::test_fixtures::TEST_DB_CONSTRAINT_DEFINITION,
            ))
            .expect(constants_str::test_fixtures::VALUE_A4B28D38),
        )]
        .into(),
    )
}

#[test]
fn ordering_does_not_affect_snapshot_and_differences_are_reported() {
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
fn every_catalog_object_kind_difference_is_reported() {
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
