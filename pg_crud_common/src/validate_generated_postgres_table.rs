use crate::*;

pub async fn validate_generated_postgres_table<Table>(
    pool: SqlxPgPoolRef<'_>,
    schema: DbSchemaNameRef<'_>,
) -> Result<(), DbSchemaConformanceError>
where
    Table: DbTableSchema,
{
    let table_columns = Table::columns();
    let mut expected = table_columns
        .iter()
        .map(|column| {
            Ok(DbColumnContractSnapshot::new(
                DbSchemaText::try_from(column.name.0.to_owned())
                    .map_err(DbSchemaConformanceError::SchemaTextTooLong)?,
                DbSchemaText::try_from(column.data_type.0.to_owned())
                    .map_err(DbSchemaConformanceError::SchemaTextTooLong)?,
                column.nullable,
                column.has_server_default,
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;
    Vec::from(Table::create_excluded_columns())
        .into_iter()
        .chain(Vec::from(Table::read_excluded_columns()))
        .chain(std::iter::once(Table::primary_key_column()))
        .chain(
            Vec::from(Table::keys())
                .into_iter()
                .flat_map(|key| match key {
                    DbKeySpec::ForeignKey { columns, .. }
                    | DbKeySpec::PrimaryKey(columns)
                    | DbKeySpec::Unique(columns) => Vec::from(columns),
                }),
        )
        .try_for_each(|field| {
            if table_columns.iter().any(|column| column.name == field) {
                Ok(())
            } else {
                let name = DbSchemaText::try_from(field.0.to_owned())
                    .map_err(DbSchemaConformanceError::SchemaTextTooLong)?;
                Err(DbSchemaConformanceError::DescriptorFieldMismatch(name))
            }
        })?;
    let rows = sqlx::query(constants_str::DB_SCHEMA_COLUMN_CONTRACT_QUERY)
        .bind(schema.0)
        .bind(Table::schema_table_text().0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    let mut observed = rows
        .into_iter()
        .map(|row| {
            let nullable: String =
                sqlx::Row::try_get(&row, constants_str::IS_NULLABLE).map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                })?;
            Ok(DbColumnContractSnapshot::new(
                DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, constants_str::COLUMN_NAME).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(DbSchemaConformanceError::SchemaTextTooLong)?,
                DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, constants_str::DATA_TYPE).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(DbSchemaConformanceError::SchemaTextTooLong)?,
                DbColumnNullable::from(nullable == constants_str::YES),
                DbColumnHasServerDefault::from(
                    sqlx::Row::try_get::<bool, _>(&row, constants_str::HAS_SERVER_DEFAULT)
                        .map_err(|error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        })?,
                ),
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;
    expected.sort_unstable();
    observed.sort_unstable();
    if expected != observed {
        return Err(DbSchemaConformanceError::ColumnContractMismatch {
            expected: expected.into(),
            observed: observed.into(),
        });
    }
    let mut expected_keys = Vec::from(Table::keys())
        .into_iter()
        .map(|key| match key {
            DbKeySpec::ForeignKey {
                columns,
                referenced_columns,
                referenced_table,
            } => Ok(DbKeyContractSnapshot::ForeignKey {
                columns: static_schema_texts(columns)?,
                referenced_columns: static_schema_texts(referenced_columns)?,
                referenced_table: static_schema_text(referenced_table)?,
            }),
            DbKeySpec::PrimaryKey(columns) => Ok(DbKeyContractSnapshot::PrimaryKey(
                static_schema_texts(columns)?,
            )),
            DbKeySpec::Unique(columns) => {
                Ok(DbKeyContractSnapshot::Unique(static_schema_texts(columns)?))
            }
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;
    let key_rows = sqlx::query(constants_str::DB_SCHEMA_KEY_CONTRACT_QUERY)
        .bind(schema.0)
        .bind(Table::schema_table_text().0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    let mut observed_keys = key_rows
        .into_iter()
        .map(|row| {
            let kind = sqlx::Row::try_get::<String, _>(&row, constants_str::CONSTRAINT_TYPE)
                .map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                })?;
            let columns = schema_texts(
                sqlx::Row::try_get::<Vec<String>, _>(&row, constants_str::COLUMNS).map_err(
                    |error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                            error,
                        ))
                    },
                )?,
            )?;
            match kind.as_str() {
                constants_str::DB_CONSTRAINT_FOREIGN_KEY_SHORT => {
                    let referenced_columns = schema_texts(
                        sqlx::Row::try_get::<Vec<String>, _>(
                            &row,
                            constants_str::REFERENCED_COLUMNS,
                        )
                        .map_err(|error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        })?,
                    )?;
                    let referenced_table = schema_text(
                        sqlx::Row::try_get::<String, _>(&row, constants_str::REFERENCED_TABLE)
                            .map_err(|error| {
                                DbSchemaConformanceError::Inspection(
                                    SqlxDbSchemaInspectionError::from(error),
                                )
                            })?,
                    )?;
                    Ok(DbKeyContractSnapshot::ForeignKey {
                        columns: columns.into(),
                        referenced_columns: referenced_columns.into(),
                        referenced_table,
                    })
                }
                constants_str::DB_CONSTRAINT_PRIMARY_KEY_SHORT => {
                    Ok(DbKeyContractSnapshot::PrimaryKey(columns.into()))
                }
                constants_str::DB_CONSTRAINT_UNIQUE_SHORT => {
                    Ok(DbKeyContractSnapshot::Unique(columns.into()))
                }
                _ => Err(DbSchemaConformanceError::UnknownObjectKind),
            }
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;
    expected_keys.sort_unstable();
    observed_keys.sort_unstable();
    if expected_keys == observed_keys {
        Ok(())
    } else {
        Err(DbSchemaConformanceError::KeyContractMismatch {
            expected: expected_keys.into(),
            observed: observed_keys.into(),
        })
    }
}
