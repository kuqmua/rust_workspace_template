const DB_SCHEMA_TEXT_MAX_LEN: usize = 1_048_576usize;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, newtype::BoundedString)]
#[bounded_string(max = DB_SCHEMA_TEXT_MAX_LEN)]
pub struct DbSchemaText(String);

#[derive(Clone, Copy, Debug)]
pub struct DbSchemaTextError(DbSchemaTextTryFromStringError);
impl std::fmt::Display for DbSchemaTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DbColumnNullable(bool);
impl From<bool> for DbColumnNullable {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SqlxPgPoolRef<'value_lt>(&'value_lt sqlx::PgPool);
impl<'value_lt> From<&'value_lt sqlx::PgPool> for SqlxPgPoolRef<'value_lt> {
    fn from(value: &'value_lt sqlx::PgPool) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DbSchemaNameRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for DbSchemaNameRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DbTableNameRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for DbTableNameRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DbColumnSnapshot {
    data_type: DbSchemaText,
    default: Option<DbSchemaText>,
    name: DbSchemaText,
    nullable: DbColumnNullable,
}
impl DbColumnSnapshot {
    #[must_use]
    pub const fn new(
        name: DbSchemaText,
        data_type: DbSchemaText,
        nullable: DbColumnNullable,
        default: Option<DbSchemaText>,
    ) -> Self {
        Self {
            data_type,
            default,
            name,
            nullable,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DbObjectKind {
    Check,
    ForeignKey,
    Index,
    PrimaryKey,
    Unique,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DbObjectSnapshot {
    definition: DbSchemaText,
    kind: DbObjectKind,
    name: DbSchemaText,
}
impl DbObjectSnapshot {
    #[must_use]
    pub const fn new(name: DbSchemaText, kind: DbObjectKind, definition: DbSchemaText) -> Self {
        Self {
            definition,
            kind,
            name,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DbTableSnapshot {
    columns: Vec<DbColumnSnapshot>,
    objects: Vec<DbObjectSnapshot>,
}
impl DbTableSnapshot {
    #[must_use]
    pub fn new(mut columns: Vec<DbColumnSnapshot>, mut objects: Vec<DbObjectSnapshot>) -> Self {
        columns.sort_unstable();
        objects.sort_unstable();
        Self { columns, objects }
    }
}

#[derive(Debug)]
pub struct SqlxDbSchemaInspectionError(sqlx::Error);
impl std::fmt::Display for SqlxDbSchemaInspectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DbSchemaConformanceError {
    #[error("failed to inspect PostgreSQL schema: {0}")]
    Inspection(SqlxDbSchemaInspectionError),
    #[error("PostgreSQL table schema differs from the expected snapshot")]
    Mismatch {
        expected: DbTableSnapshot,
        observed: DbTableSnapshot,
    },
    #[error("PostgreSQL schema text exceeds the supported limit")]
    SchemaTextTooLong(DbSchemaTextError),
}

pub fn validate_postgres_table_schema(
    expected: DbTableSnapshot,
    observed: DbTableSnapshot,
) -> Result<(), DbSchemaConformanceError> {
    if expected == observed {
        Ok(())
    } else {
        Err(DbSchemaConformanceError::Mismatch { expected, observed })
    }
}

pub async fn inspect_postgres_table(
    pool: SqlxPgPoolRef<'_>,
    schema: DbSchemaNameRef<'_>,
    table: DbTableNameRef<'_>,
) -> Result<DbTableSnapshot, DbSchemaConformanceError> {
    let column_rows = sqlx::query(str_constants::DB_SCHEMA_COLUMN_QUERY)
        .bind(schema.0)
        .bind(table.0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
        })?;
    let columns = column_rows
        .into_iter()
        .map(|row| {
            let nullable: String =
                sqlx::Row::try_get(&row, str_constants::IS_NULLABLE).map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                })?;
            let default: Option<String> = sqlx::Row::try_get(&row, str_constants::COLUMN_DEFAULT)
                .map_err(|error| {
                DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
            })?;
            Ok(DbColumnSnapshot::new(
                DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, str_constants::COLUMN_NAME).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                        },
                    )?,
                )
                .map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                })?,
                DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, str_constants::DATA_TYPE).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                        },
                    )?,
                )
                .map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                })?,
                (nullable == str_constants::YES).into(),
                default
                    .map(DbSchemaText::try_from)
                    .transpose()
                    .map_err(|error| {
                        DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                    })?,
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;

    let constraint_rows = sqlx::query(str_constants::DB_SCHEMA_CONSTRAINT_QUERY)
        .bind(schema.0)
        .bind(table.0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
        })?;
    let mut objects = constraint_rows
        .into_iter()
        .map(|row| {
            let constraint_type: String = sqlx::Row::try_get(&row, str_constants::CONSTRAINT_TYPE)
                .map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                })?;
            let kind = match constraint_type.as_str() {
                str_constants::DB_CONSTRAINT_CHECK => DbObjectKind::Check,
                str_constants::DB_CONSTRAINT_FOREIGN_KEY => DbObjectKind::ForeignKey,
                str_constants::DB_CONSTRAINT_PRIMARY_KEY => DbObjectKind::PrimaryKey,
                str_constants::DB_CONSTRAINT_UNIQUE => DbObjectKind::Unique,
                _ => return Ok(None),
            };
            Ok(Some(
                DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, str_constants::CONSTRAINT_NAME).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                        },
                    )?,
                )
                .map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                })
                .and_then(|name| {
                    DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, str_constants::CONSTRAINT_DEFINITION)
                            .map_err(|error| {
                                DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(
                                    error,
                                ))
                            })?,
                    )
                    .map_err(|error| {
                        DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                    })
                    .map(|definition| DbObjectSnapshot::new(name, kind, definition))
                })?,
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let index_rows = sqlx::query(str_constants::DB_SCHEMA_INDEX_QUERY)
        .bind(schema.0)
        .bind(table.0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
        })?;
    objects.extend(
        index_rows
            .into_iter()
            .map(|row| {
                Ok(DbObjectSnapshot::new(
                    DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, str_constants::INDEX_NAME).map_err(
                            |error| {
                                DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(
                                    error,
                                ))
                            },
                        )?,
                    )
                    .map_err(|error| {
                        DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                    })?,
                    DbObjectKind::Index,
                    DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, str_constants::INDEX_DEFINITION)
                            .map_err(|error| {
                                DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(
                                    error,
                                ))
                            })?,
                    )
                    .map_err(|error| {
                        DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                    })?,
                ))
            })
            .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?,
    );
    Ok(DbTableSnapshot::new(columns, objects))
}

#[cfg(test)]
mod tests {
    fn snapshot(nullable: bool) -> super::DbTableSnapshot {
        super::DbTableSnapshot::new(
            vec![super::DbColumnSnapshot::new(
                super::DbSchemaText::try_from(String::from(str_constants::TEST_DB_COLUMN_ID))
                    .expect(str_constants::VALUE_11F0D7F5),
                super::DbSchemaText::try_from(String::from(str_constants::TEST_DB_DATA_TYPE_UUID))
                    .expect(str_constants::VALUE_9CB64C93),
                nullable.into(),
                None,
            )],
            vec![super::DbObjectSnapshot::new(
                super::DbSchemaText::try_from(String::from(str_constants::TEST_DB_CONSTRAINT_NAME))
                    .expect(str_constants::VALUE_61F95647),
                super::DbObjectKind::PrimaryKey,
                super::DbSchemaText::try_from(String::from(
                    str_constants::TEST_DB_CONSTRAINT_DEFINITION,
                ))
                .expect(str_constants::VALUE_A4B28D38),
            )],
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
}
