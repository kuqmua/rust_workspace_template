pub async fn validate_generated_postgres_table<Table>(
    sqlx_pg_catalog_pool_ref: crate::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef<'_>,
    db_schema_name_ref: crate::db_schema_name_ref::DbSchemaNameRef<'_>,
) -> Result<(), crate::db_schema_conformance_error::DbSchemaConformanceError>
where
    Table: crate::db_table_schema::DbTableSchema,
{
    let table_columns = Table::columns();
    let mut expected = table_columns
        .iter()
        .map(|column| {
            Ok(crate::db_column_contract_snapshot::DbColumnContractSnapshot::new(
                crate::db_schema_text::DbSchemaText::try_from((*column.get_name().get_inner()).to_owned())
                    .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)?,
                crate::db_schema_text::DbSchemaText::try_from((*column.get_data_type().get_inner()).to_owned())
                    .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)?,
                *column.get_nullable(),
                *column.get_has_server_default(),
            ))
        })
        .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>()?;
    Vec::from(Table::create_excluded_columns())
        .into_iter()
        .chain(Vec::from(Table::read_excluded_columns()))
        .chain(std::iter::once(Table::primary_key_column()))
        .chain(
            Vec::from(Table::keys())
                .into_iter()
                .flat_map(|key| match key {
                    crate::db_key_spec::DbKeySpec::ForeignKey { columns, .. }
                    | crate::db_key_spec::DbKeySpec::PrimaryKey(columns)
                    | crate::db_key_spec::DbKeySpec::Unique(columns) => Vec::from(columns),
                }),
        )
        .try_for_each(|field| {
            if table_columns.iter().any(|column| *column.get_name() == field) {
                Ok(())
            } else {
                let name = crate::db_schema_text::DbSchemaText::try_from((*field.get_inner()).to_owned())
                    .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)?;
                Err(crate::db_schema_conformance_error::DbSchemaConformanceError::DescriptorFieldMismatch(name))
            }
        })?;
    let rows = sqlx::query(constants_str::DB_SCHEMA_COLUMN_CONTRACT_QUERY)
        .bind(*db_schema_name_ref.get_inner())
        .bind(*Table::schema_table_text().get_inner())
        .fetch_all(*sqlx_pg_catalog_pool_ref.get_inner())
        .await
        .map_err(|error| {
            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
            )
        })?;
    let mut observed = rows
        .into_iter()
        .map(|row| {
            let nullable: crate::db_schema_text::DbSchemaText =
                crate::db_schema_text::DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, constants_str::IS_NULLABLE).map_err(|error| {
                    crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error))
                })?,
                )
                .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)?;
            Ok(crate::db_column_contract_snapshot::DbColumnContractSnapshot::new(
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
                crate::db_column_nullable::DbColumnNullable::from(nullable.as_ref() == constants_str::YES),
                crate::db_column_has_server_default::DbColumnHasServerDefault::from(
                    sqlx::Row::try_get::<bool, _>(&row, constants_str::HAS_SERVER_DEFAULT)
                        .map_err(|error| {
                            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        })?,
                ),
            ))
        })
        .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>()?;
    expected.sort();
    observed.sort();
    if expected != observed {
        return Err(
            crate::db_schema_conformance_error::DbSchemaConformanceError::ColumnContractMismatch {
                expected: expected.into(),
                observed: observed.into(),
            },
        );
    }
    let mut expected_keys = Vec::from(Table::keys())
        .into_iter()
        .map(|key| match key {
            crate::db_key_spec::DbKeySpec::ForeignKey {
                columns,
                referenced_columns,
                referenced_table,
            } => Ok(
                crate::db_key_contract_snapshot::DbKeyContractSnapshot::ForeignKey {
                    columns: crate::static_schema_texts::static_schema_texts(columns)?,
                    referenced_columns: crate::static_schema_texts::static_schema_texts(
                        referenced_columns,
                    )?,
                    referenced_table: crate::static_schema_text::static_schema_text(
                        referenced_table,
                    )?,
                },
            ),
            crate::db_key_spec::DbKeySpec::PrimaryKey(columns) => Ok(
                crate::db_key_contract_snapshot::DbKeyContractSnapshot::PrimaryKey(
                    crate::static_schema_texts::static_schema_texts(columns)?,
                ),
            ),
            crate::db_key_spec::DbKeySpec::Unique(columns) => Ok(
                crate::db_key_contract_snapshot::DbKeyContractSnapshot::Unique(
                    crate::static_schema_texts::static_schema_texts(columns)?,
                ),
            ),
        })
        .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>(
        )?;
    let key_rows = sqlx::query(constants_str::DB_SCHEMA_KEY_CONTRACT_QUERY)
        .bind(*db_schema_name_ref.get_inner())
        .bind(*Table::schema_table_text().get_inner())
        .fetch_all(*sqlx_pg_catalog_pool_ref.get_inner())
        .await
        .map_err(|error| {
            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
            )
        })?;
    let mut observed_keys = key_rows
        .into_iter()
        .map(|row| {
            let kind = sqlx::Row::try_get::<String, _>(&row, constants_str::CONSTRAINT_TYPE)
                .map_err(|error| {
                    crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error))
                })?;
            let columns = crate::schema_texts::schema_texts(
                sqlx::Row::try_get::<Vec<String>, _>(&row, constants_str::COLUMNS).map_err(
                    |error| {
                        crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                            error,
                        ))
                    },
                )?,
            )?;
            match kind.as_str() {
                constants_str::DB_CONSTRAINT_FOREIGN_KEY_SHORT => {
                    let referenced_columns = crate::schema_texts::schema_texts(
                        sqlx::Row::try_get::<Vec<String>, _>(
                            &row,
                            constants_str::REFERENCED_COLUMNS,
                        )
                        .map_err(|error| {
                            crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        })?,
                    )?;
                    let referenced_table = crate::schema_text::schema_text(
                        sqlx::Row::try_get::<String, _>(&row, constants_str::REFERENCED_TABLE)
                            .map_err(|error| {
                                crate::db_schema_conformance_error::DbSchemaConformanceError::Inspection(
                                    crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError::from(error),
                                )
                            })?,
                    )?;
                    Ok(crate::db_key_contract_snapshot::DbKeyContractSnapshot::ForeignKey {
                        columns: columns.into(),
                        referenced_columns: referenced_columns.into(),
                        referenced_table,
                    })
                }
                constants_str::DB_CONSTRAINT_PRIMARY_KEY_SHORT => {
                    Ok(crate::db_key_contract_snapshot::DbKeyContractSnapshot::PrimaryKey(columns.into()))
                }
                constants_str::DB_CONSTRAINT_UNIQUE_SHORT => {
                    Ok(crate::db_key_contract_snapshot::DbKeyContractSnapshot::Unique(columns.into()))
                }
                _ => Err(crate::db_schema_conformance_error::DbSchemaConformanceError::UnknownObjectKind),
            }
        })
        .collect::<Result<Vec<_>, crate::db_schema_conformance_error::DbSchemaConformanceError>>()?;
    expected_keys.sort();
    observed_keys.sort();
    if expected_keys == observed_keys {
        Ok(())
    } else {
        Err(
            crate::db_schema_conformance_error::DbSchemaConformanceError::KeyContractMismatch {
                expected: expected_keys.into(),
                observed: observed_keys.into(),
            },
        )
    }
}
