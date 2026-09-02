pub async fn validate_postgres_table_extensions<Table>(
    sqlx_pg_catalog_pool_ref: crate::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef<'_>,
    db_schema_name_ref: crate::db_schema_name_ref::DbSchemaNameRef<'_>,
) -> Result<(), crate::db_schema_conformance_error::DbSchemaConformanceError>
where
    Table: crate::db_extended_table_schema::DbExtendedTableSchema,
{
    let mut expected_defaults = Table::exact_defaults()
        .iter()
        .map(|spec| {
            Ok(crate::db_object_snapshot::DbObjectSnapshot::new(
                crate::schema_text::schema_text((*spec.get_column().get_inner()).to_owned())?,
                crate::db_object_kind::DbObjectKind::Default,
                crate::schema_text::schema_text((*spec.get_expression().get_inner()).to_owned())?,
            ))
        })
        .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>(
        )?;
    let default_rows = sqlx::query(constants_str::DB_SCHEMA_EXACT_DEFAULT_QUERY)
        .bind(*db_schema_name_ref.get_inner())
        .bind(*Table::schema_table_text().get_inner())
        .fetch_all(*sqlx_pg_catalog_pool_ref.get_inner())
        .await
        .map_err(|error| {
            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
            )
        })?;
    let mut observed_defaults = default_rows
        .into_iter()
        .map(|row| {
            Ok(crate::db_object_snapshot::DbObjectSnapshot::new(
                crate::schema_text::schema_text(
                    sqlx::Row::try_get(&row, constants_str::COLUMN_NAME).map_err(|error| {
                        crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                            error,
                        ))
                    })?,
                )?,
                crate::db_object_kind::DbObjectKind::Default,
                crate::schema_text::schema_text(
                    sqlx::Row::try_get(&row, constants_str::COLUMN_DEFAULT).map_err(|error| {
                        crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                            error,
                        ))
                    })?,
                )?,
            ))
        })
        .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>()?;
    expected_defaults.sort();
    observed_defaults.sort();
    if expected_defaults != observed_defaults {
        return Err(
            crate::db_schema_conformance_error::DbSchemaConformanceError::DefaultContractMismatch {
                expected: expected_defaults.into(),
                observed: observed_defaults.into(),
            },
        );
    }
    let public_schema_qualifier = format!("{}.", constants_str::PUBLIC);
    let observed_schema_qualifier = format!("{}.", *db_schema_name_ref.get_inner());
    let mut expected_objects = Table::checks_and_indexes()
        .iter()
        .map(|spec| {
            Ok(crate::db_object_snapshot::DbObjectSnapshot::new(
                crate::schema_text::schema_text((*spec.get_name().get_inner()).to_owned())?,
                *spec.get_kind(),
                crate::schema_text::schema_text(spec.get_definition().get_inner().replace(
                    public_schema_qualifier.as_str(),
                    observed_schema_qualifier.as_str(),
                ))?,
            ))
        })
        .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>(
        )?;
    let rows = sqlx::query(constants_str::DB_SCHEMA_CHECK_AND_NON_CONSTRAINT_INDEX_QUERY)
        .bind(*db_schema_name_ref.get_inner())
        .bind(*Table::schema_table_text().get_inner())
        .fetch_all(*sqlx_pg_catalog_pool_ref.get_inner())
        .await
        .map_err(|error| {
            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
            )
        })?;
    let mut observed_objects = rows
        .into_iter()
        .map(|row| {
            let kind = match sqlx::Row::try_get::<String, _>(&row, constants_str::OBJECT_KIND)
                .map_err(|error| {
                    crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error))
                })?
                .as_str()
            {
                constants_str::CHECK => crate::db_object_kind::DbObjectKind::Check,
                constants_str::INDEX => crate::db_object_kind::DbObjectKind::Index,
                _ => return Err(crate::db_schema_conformance_error::DbSchemaConformanceError::UnknownObjectKind),
            };
            Ok(crate::db_object_snapshot::DbObjectSnapshot::new(
                crate::schema_text::schema_text(
                    sqlx::Row::try_get(&row, constants_str::OBJECT_NAME).map_err(|error| {
                        crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                            error,
                        ))
                    })?,
                )?,
                kind,
                crate::schema_text::schema_text(
                    sqlx::Row::try_get(&row, constants_str::OBJECT_DEFINITION).map_err(
                        |error| {
                            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )?,
            ))
        })
        .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>()?;
    expected_objects.sort();
    observed_objects.sort();
    if expected_objects == observed_objects {
        Ok(())
    } else {
        Err(crate::db_schema_conformance_error::DbSchemaConformanceError::ExtendedObjectContractMismatch {
            expected: expected_objects.into(),
            observed: observed_objects.into(),
        })
    }
}
