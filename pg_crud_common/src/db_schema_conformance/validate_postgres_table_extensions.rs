use super::*;

pub async fn validate_postgres_table_extensions<Table>(
    pool: SqlxPgPoolRef<'_>,
    schema: DbSchemaNameRef<'_>,
) -> Result<(), DbSchemaConformanceError>
where
    Table: DbExtendedTableSchema,
{
    let mut expected_defaults = Table::exact_defaults()
        .iter()
        .map(|spec| {
            Ok(DbObjectSnapshot::new(
                schema_text(spec.column.0.to_owned())?,
                DbObjectKind::Default,
                schema_text(spec.expression.0.to_owned())?,
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;
    let default_rows = sqlx::query(constants_str::DB_SCHEMA_EXACT_DEFAULT_QUERY)
        .bind(schema.0)
        .bind(Table::schema_table_text().0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    let mut observed_defaults = default_rows
        .into_iter()
        .map(|row| {
            Ok(DbObjectSnapshot::new(
                schema_text(
                    sqlx::Row::try_get(&row, constants_str::COLUMN_NAME).map_err(|error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                            error,
                        ))
                    })?,
                )?,
                DbObjectKind::Default,
                schema_text(
                    sqlx::Row::try_get(&row, constants_str::COLUMN_DEFAULT).map_err(|error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                            error,
                        ))
                    })?,
                )?,
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;
    expected_defaults.sort_unstable();
    observed_defaults.sort_unstable();
    if expected_defaults != observed_defaults {
        return Err(DbSchemaConformanceError::DefaultContractMismatch {
            expected: expected_defaults.into(),
            observed: observed_defaults.into(),
        });
    }
    let public_schema_qualifier = format!("{}.", constants_str::PUBLIC);
    let observed_schema_qualifier = format!("{}.", schema.0);
    let mut expected_objects = Table::checks_and_indexes()
        .iter()
        .map(|spec| {
            Ok(DbObjectSnapshot::new(
                schema_text(spec.name.0.to_owned())?,
                spec.kind,
                schema_text(spec.definition.0.replace(
                    public_schema_qualifier.as_str(),
                    observed_schema_qualifier.as_str(),
                ))?,
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;
    let rows = sqlx::query(constants_str::DB_SCHEMA_CHECK_AND_NON_CONSTRAINT_INDEX_QUERY)
        .bind(schema.0)
        .bind(Table::schema_table_text().0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    let mut observed_objects = rows
        .into_iter()
        .map(|row| {
            let kind = match sqlx::Row::try_get::<String, _>(&row, constants_str::OBJECT_KIND)
                .map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                })?
                .as_str()
            {
                constants_str::CHECK => DbObjectKind::Check,
                constants_str::INDEX => DbObjectKind::Index,
                _ => return Err(DbSchemaConformanceError::UnknownObjectKind),
            };
            Ok(DbObjectSnapshot::new(
                schema_text(
                    sqlx::Row::try_get(&row, constants_str::OBJECT_NAME).map_err(|error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                            error,
                        ))
                    })?,
                )?,
                kind,
                schema_text(
                    sqlx::Row::try_get(&row, constants_str::OBJECT_DEFINITION).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )?,
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;
    expected_objects.sort_unstable();
    observed_objects.sort_unstable();
    if expected_objects == observed_objects {
        Ok(())
    } else {
        Err(DbSchemaConformanceError::ExtendedObjectContractMismatch {
            expected: expected_objects.into(),
            observed: observed_objects.into(),
        })
    }
}
