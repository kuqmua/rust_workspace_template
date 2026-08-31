pub async fn inspect_postgres_catalog(
    pool: crate::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef<'_>,
    schema: crate::db_schema_name_ref::DbSchemaNameRef<'_>,
) -> Result<
    crate::db_catalog_snapshot::DbCatalogSnapshot,
    crate::db_schema_conformance_error::DbSchemaConformanceError,
> {
    let rows = sqlx::query(constants_str::DB_SCHEMA_CATALOG_QUERY)
        .bind(schema.0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
            )
        })?;
    rows.into_iter()
        .map(|row| {
            let kind_text: String =
                sqlx::Row::try_get(&row, constants_str::OBJECT_KIND).map_err(|error| {
                    crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                        crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                            error,
                        ),
                    )
                })?;
            let kind = match kind_text.as_str() {
                constants_str::EXTENSION => crate::db_object_kind::DbObjectKind::Extension,
                constants_str::FUNCTION => crate::db_object_kind::DbObjectKind::Function,
                constants_str::TRIGGER => crate::db_object_kind::DbObjectKind::Trigger,
                constants_str::VIEW => crate::db_object_kind::DbObjectKind::View,
                _ => return Err(
                    crate::db_schema_conformance_error::DbSchemaConformanceError::UnknownObjectKind,
                ),
            };
            let name = sqlx::Row::try_get::<String, _>(&row, constants_str::OBJECT_NAME).map_err(
                |error| {
                    crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                        crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                            error,
                        ),
                    )
                },
            )?;
            let definition =
                sqlx::Row::try_get::<String, _>(&row, constants_str::OBJECT_DEFINITION).map_err(
                    |error| {
                        crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                    crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                        error,
                    ),
                )
                    },
                )?;
            Ok(crate::db_object_snapshot::DbObjectSnapshot::new(
                crate::db_schema_text::DbSchemaText::try_from(name).map_err(
                    crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong,
                )?,
                kind,
                crate::db_schema_text::DbSchemaText::try_from(definition).map_err(
                    crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong,
                )?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|objects| crate::db_catalog_snapshot::DbCatalogSnapshot::new(objects.into()))
}
