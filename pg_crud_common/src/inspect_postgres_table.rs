use crate::*;

pub async fn inspect_postgres_table(
    pool: SqlxPgPoolRef<'_>,
    schema: DbSchemaNameRef<'_>,
    table: DbTableNameRef<'_>,
) -> Result<DbTableSnapshot, DbSchemaConformanceError> {
    let column_rows = sqlx::query(constants_str::DB_SCHEMA_COLUMN_QUERY)
        .bind(schema.0)
        .bind(table.0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    let columns = column_rows
        .into_iter()
        .map(|row| {
            let nullable: String =
                sqlx::Row::try_get(&row, constants_str::IS_NULLABLE).map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                })?;
            let default: Option<String> = sqlx::Row::try_get(&row, constants_str::COLUMN_DEFAULT)
                .map_err(|error| {
                DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
            })?;
            Ok(DbColumnSnapshot::new(
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
                (nullable == constants_str::YES).into(),
                default
                    .map(DbSchemaText::try_from)
                    .transpose()
                    .map_err(DbSchemaConformanceError::SchemaTextTooLong)?,
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;

    let constraint_rows = sqlx::query(constants_str::DB_SCHEMA_CONSTRAINT_QUERY)
        .bind(schema.0)
        .bind(table.0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    let mut objects = constraint_rows
        .into_iter()
        .map(|row| {
            let constraint_type: String = sqlx::Row::try_get(&row, constants_str::CONSTRAINT_TYPE)
                .map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                })?;
            let kind = match constraint_type.as_str() {
                constants_str::DB_CONSTRAINT_CHECK => DbObjectKind::Check,
                constants_str::DB_CONSTRAINT_FOREIGN_KEY => DbObjectKind::ForeignKey,
                constants_str::DB_CONSTRAINT_PRIMARY_KEY => DbObjectKind::PrimaryKey,
                constants_str::DB_CONSTRAINT_UNIQUE => DbObjectKind::Unique,
                _ => return Ok(None),
            };
            Ok(Some(
                DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, constants_str::CONSTRAINT_NAME).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(DbSchemaConformanceError::SchemaTextTooLong)
                .and_then(|name| {
                    DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, constants_str::CONSTRAINT_DEFINITION)
                            .map_err(|error| {
                                DbSchemaConformanceError::Inspection(
                                    SqlxDbSchemaInspectionError::from(error),
                                )
                            })?,
                    )
                    .map_err(DbSchemaConformanceError::SchemaTextTooLong)
                    .map(|definition| DbObjectSnapshot::new(name, kind, definition))
                })?,
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let index_rows = sqlx::query(constants_str::DB_SCHEMA_INDEX_QUERY)
        .bind(schema.0)
        .bind(table.0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    objects.extend(
        index_rows
            .into_iter()
            .map(|row| {
                Ok(DbObjectSnapshot::new(
                    DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, constants_str::INDEX_NAME).map_err(
                            |error| {
                                DbSchemaConformanceError::Inspection(
                                    SqlxDbSchemaInspectionError::from(error),
                                )
                            },
                        )?,
                    )
                    .map_err(DbSchemaConformanceError::SchemaTextTooLong)?,
                    DbObjectKind::Index,
                    DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, constants_str::INDEX_DEFINITION)
                            .map_err(|error| {
                                DbSchemaConformanceError::Inspection(
                                    SqlxDbSchemaInspectionError::from(error),
                                )
                            })?,
                    )
                    .map_err(DbSchemaConformanceError::SchemaTextTooLong)?,
                ))
            })
            .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?,
    );
    Ok(DbTableSnapshot::new(columns.into(), objects.into()))
}
