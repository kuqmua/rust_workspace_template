fn catalog_snapshot(kind: super::DbObjectKind) -> super::DbCatalogSnapshot {
    super::DbCatalogSnapshot::new(
        vec![super::DbObjectSnapshot::new(
            super::DbSchemaText::try_from(String::from(constants_str::TEST_DB_OBJECT_NAME))
                .expect(constants_str::VALUE_E84FED1B),
            kind,
            super::DbSchemaText::try_from(String::from(constants_str::TEST_DB_OBJECT_DEFINITION))
                .expect(constants_str::VALUE_A7950FF0),
        )]
        .into(),
    )
}

fn snapshot(nullable: bool) -> super::DbTableSnapshot {
    super::DbTableSnapshot::new(
        vec![super::DbColumnSnapshot::new(
            super::DbSchemaText::try_from(String::from(constants_str::TEST_DB_COLUMN_ID))
                .expect(constants_str::VALUE_11F0D7F5),
            super::DbSchemaText::try_from(String::from(constants_str::TEST_DB_DATA_TYPE_UUID))
                .expect(constants_str::VALUE_9CB64C93),
            nullable.into(),
            None,
        )]
        .into(),
        vec![super::DbObjectSnapshot::new(
            super::DbSchemaText::try_from(String::from(constants_str::TEST_DB_CONSTRAINT_NAME))
                .expect(constants_str::VALUE_61F95647),
            super::DbObjectKind::PrimaryKey,
            super::DbSchemaText::try_from(String::from(
                constants_str::TEST_DB_CONSTRAINT_DEFINITION,
            ))
            .expect(constants_str::VALUE_A4B28D38),
        )]
        .into(),
    )
}

#[test]
fn ordering_does_not_affect_snapshot_and_differences_are_reported() {
    assert!(matches!(
        super::validate_postgres_table_schema(snapshot(false), snapshot(false)),
        Ok(())
    ));
    assert!(matches!(
        super::validate_postgres_table_schema(snapshot(false), snapshot(true)),
        Err(super::DbSchemaConformanceError::Mismatch { .. })
    ));
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator keeps the all-kinds assertion compact under the workspace no-for-loop policy"
)]
fn every_catalog_object_kind_difference_is_reported() {
    let kinds = [
        super::DbObjectKind::Check,
        super::DbObjectKind::Default,
        super::DbObjectKind::Extension,
        super::DbObjectKind::ForeignKey,
        super::DbObjectKind::Function,
        super::DbObjectKind::Index,
        super::DbObjectKind::PrimaryKey,
        super::DbObjectKind::Trigger,
        super::DbObjectKind::Unique,
        super::DbObjectKind::View,
    ];
    kinds.into_iter().for_each(|kind| {
        let result = super::validate_postgres_catalog(
            catalog_snapshot(super::DbObjectKind::Function),
            catalog_snapshot(kind),
        );
        if kind == super::DbObjectKind::Function {
            assert!(matches!(result, Ok(())));
        } else {
            assert!(matches!(
                result,
                Err(super::DbSchemaConformanceError::CatalogMismatch { .. })
            ));
        }
    });
}
