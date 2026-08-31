pub async fn inspect_postgres_table(
    pool: crate::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef<'_>,
    schema: crate::db_schema_name_ref::DbSchemaNameRef<'_>,
    table: crate::db_table_name_ref::DbTableNameRef<'_>,
) -> Result<
    crate::db_table_snapshot::DbTableSnapshot,
    crate::db_schema_conformance_error::DbSchemaConformanceError,
> {
    let column_rows = sqlx::query(constants_str::DB_SCHEMA_COLUMN_QUERY)
        .bind(*schema.get_inner())
        .bind(*table.get_inner())
        .fetch_all(*pool.get_inner())
        .await
        .map_err(|error| {
            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
            )
        })?;
    let columns = column_rows
        .into_iter()
        .map(|row| {
            let nullable: String =
                sqlx::Row::try_get(&row, constants_str::IS_NULLABLE).map_err(|error| {
                    crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error))
                })?;
            let default: Option<String> = sqlx::Row::try_get(&row, constants_str::COLUMN_DEFAULT)
                .map_err(|error| {
                crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error))
            })?;
            Ok(crate::db_column_snapshot::DbColumnSnapshot::new(
                crate::db_schema_text::DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, constants_str::COLUMN_NAME).map_err(
                        |error| {
                            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)?,
                crate::db_schema_text::DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, constants_str::DATA_TYPE).map_err(
                        |error| {
                            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)?,
                (nullable == constants_str::YES).into(),
                default
                    .map(crate::db_schema_text::DbSchemaText::try_from)
                    .transpose()
                    .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)?,
            ))
        })
        .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>()?;

    let constraint_rows = sqlx::query(constants_str::DB_SCHEMA_CONSTRAINT_QUERY)
        .bind(*schema.get_inner())
        .bind(*table.get_inner())
        .fetch_all(*pool.get_inner())
        .await
        .map_err(|error| {
            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
            )
        })?;
    let mut objects = constraint_rows
        .into_iter()
        .map(|row| {
            let constraint_type: String = sqlx::Row::try_get(&row, constants_str::CONSTRAINT_TYPE)
                .map_err(|error| {
                    crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error))
                })?;
            let kind = match constraint_type.as_str() {
                constants_str::DB_CONSTRAINT_CHECK => crate::db_object_kind::DbObjectKind::Check,
                constants_str::DB_CONSTRAINT_FOREIGN_KEY => crate::db_object_kind::DbObjectKind::ForeignKey,
                constants_str::DB_CONSTRAINT_PRIMARY_KEY => crate::db_object_kind::DbObjectKind::PrimaryKey,
                constants_str::DB_CONSTRAINT_UNIQUE => crate::db_object_kind::DbObjectKind::Unique,
                _ => return Ok(None),
            };
            Ok(Some(
                crate::db_schema_text::DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, constants_str::CONSTRAINT_NAME).map_err(
                        |error| {
                            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)
                .and_then(|name| {
                    crate::db_schema_text::DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, constants_str::CONSTRAINT_DEFINITION)
                            .map_err(|error| {
                                crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                                    crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
                                )
                            })?,
                    )
                    .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)
                    .map(|definition| crate::db_object_snapshot::DbObjectSnapshot::new(name, kind, definition))
                })?,
            ))
        })
        .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let index_rows = sqlx::query(constants_str::DB_SCHEMA_INDEX_QUERY)
        .bind(*schema.get_inner())
        .bind(*table.get_inner())
        .fetch_all(*pool.get_inner())
        .await
        .map_err(|error| {
            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
            )
        })?;
    objects.extend(
        index_rows
            .into_iter()
            .map(|row| {
                Ok(crate::db_object_snapshot::DbObjectSnapshot::new(
                    crate::db_schema_text::DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, constants_str::INDEX_NAME).map_err(
                            |error| {
                                crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                                    crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
                                )
                            },
                        )?,
                    )
                    .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)?,
                    crate::db_object_kind::DbObjectKind::Index,
                    crate::db_schema_text::DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, constants_str::INDEX_DEFINITION)
                            .map_err(|error| {
                                crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                                    crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
                                )
                            })?,
                    )
                    .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)?,
                ))
            })
            .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>()?,
    );
    Ok(crate::db_table_snapshot::DbTableSnapshot::new(
        columns.into(),
        objects.into(),
    ))
}
