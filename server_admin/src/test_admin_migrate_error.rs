#[test]
fn test_admin_migration_conversion_preserves_version_and_diagnostic() {
    [1i64, 7i64, i64::MAX].into_iter().for_each(|version| {
        let source = crate::sqlx_admin_migrate_error::SqlxAdminMigrateError::from(
            sqlx::migrate::MigrateError::VersionMissing(version),
        );
        let expected = format!(
            "{}: {source:?}",
            constants_str::ADMIN_DIAGNOSTIC_FAILED_TO_PREPARE_ADMINISTRATOR_SCHEMA_MIGRATION_FAILED,
        );
        let error = crate::admin_migrate_error::AdminMigrateError::from(source);
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_none());
        assert!(matches!(error, crate::admin_migrate_error::AdminMigrateError::Migration(migration_source)
            if matches!(migration_source.get_inner(), sqlx::migrate::MigrateError::VersionMissing(actual) if *actual == version)));
    });
}

#[test]
fn test_admin_reconciliation_conversion_preserves_decode_error_and_diagnostic() {
    let original =
        server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error::Invalid;
    let source = crate::sqlx_admin_error::SqlxAdminError::from(original);
    let expected = format!(
        "{}: {source:?}",
        constants_str::ADMIN_DIAGNOSTIC_FAILED_TO_PREPARE_ADMINISTRATOR_SCHEMA_RULE_RECONCILIATION_FAILED,
    );
    let error = crate::admin_migrate_error::AdminMigrateError::from(source);
    assert_eq!(error.to_string(), expected);
    assert!(std::error::Error::source(&error).is_none());
    assert!(
        matches!(error, crate::admin_migrate_error::AdminMigrateError::Reconciliation(reconciliation_source)
        if matches!(reconciliation_source.get_inner(), sqlx::Error::Decode(inner)
            if inner.downcast_ref::<server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error>() == Some(&original)))
    );
}

#[test]
fn test_admin_idempotency_error_preserves_nested_sqlx_source_and_diagnostic() {
    let idempotency_source =
        pg_table::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from(
            sqlx::Error::RowNotFound,
        );
    let expected = format!(
        "{}: {}",
        constants_str::ADMIN_IDEMPOTENCY_PREPARATION_FAILED,
        constants_str::POSTGRESQL_IDEMPOTENCY_OPERATION_FAILED,
    );
    let error = crate::admin_migrate_error::AdminMigrateError::Idempotency(idempotency_source);
    assert_eq!(error.to_string(), expected);
    let source = std::error::Error::source(&error).and_then(|source| {
        source
            .downcast_ref::<pg_table::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError>(
            )
    });
    assert!(source.is_some());
    assert!(matches!(
        source
            .and_then(std::error::Error::source)
            .and_then(|source| source.downcast_ref::<sqlx::Error>()),
        Some(sqlx::Error::RowNotFound)
    ));
}
