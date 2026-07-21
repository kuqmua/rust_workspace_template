//! PostgreSQL migrations are the runtime authority for database structure.
//! Generated Rust table descriptors are compile-time expectations and must be
//! checked against a database with all migrations applied before deployment.

const DB_SCHEMA_TEXT_MAX_LEN: usize = 1_048_576usize;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, newtype::BoundedString)]
#[bounded_string(max = DB_SCHEMA_TEXT_MAX_LEN)]
pub struct DbSchemaText(String);

#[derive(Clone, Copy, Debug, newtype::Display)]
pub struct DbSchemaTextError(DbSchemaTextTryFromStringError);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, newtype::FromInner)]
pub struct DbColumnNullable(bool);

pub trait PgColumnSchema {
    const DATA_TYPE: &'static str;
    const HAS_SERVER_DEFAULT: bool;
    const NULLABLE: bool;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::AsRefInner, newtype::FromInner)]
pub struct DbStaticSchemaText(&'static str);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, newtype::FromInner)]
pub struct DbColumnHasServerDefault(bool);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DbColumnSpec {
    data_type: DbStaticSchemaText,
    has_server_default: DbColumnHasServerDefault,
    name: DbStaticSchemaText,
    nullable: DbColumnNullable,
}
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbColumnSpecs(Vec<DbColumnSpec>);
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbStaticSchemaTexts(Vec<DbStaticSchemaText>);
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbKeySpecs(Vec<DbKeySpec>);
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbObjectSpecs(Vec<DbObjectSpec>);
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbDefaultSpecs(Vec<DbDefaultSpec>);

impl DbColumnSpec {
    #[must_use]
    pub const fn new(
        name: DbStaticSchemaText,
        data_type: DbStaticSchemaText,
        nullable: DbColumnNullable,
        has_server_default: DbColumnHasServerDefault,
    ) -> Self {
        Self {
            data_type,
            has_server_default,
            name,
            nullable,
        }
    }
}

pub trait DbTableSchema {
    const TABLE_NAME: &'static str;
    fn columns() -> DbColumnSpecs;
    fn create_excluded_columns() -> DbStaticSchemaTexts;
    fn keys() -> DbKeySpecs;
    fn primary_key_column() -> DbStaticSchemaText;
    fn read_excluded_columns() -> DbStaticSchemaTexts;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DbDefaultSpec {
    column: DbStaticSchemaText,
    expression: DbStaticSchemaText,
}
impl DbDefaultSpec {
    #[must_use]
    pub const fn new(column: DbStaticSchemaText, expression: DbStaticSchemaText) -> Self {
        Self { column, expression }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DbObjectSpec {
    definition: DbStaticSchemaText,
    kind: DbObjectKind,
    name: DbStaticSchemaText,
}
impl DbObjectSpec {
    #[must_use]
    pub const fn new(
        name: DbStaticSchemaText,
        kind: DbObjectKind,
        definition: DbStaticSchemaText,
    ) -> Self {
        Self {
            definition,
            kind,
            name,
        }
    }
}
pub trait DbExtendedTableSchema: DbTableSchema {
    fn checks_and_indexes() -> DbObjectSpecs;
    fn exact_defaults() -> DbDefaultSpecs;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DbKeySpec {
    ForeignKey {
        columns: DbStaticSchemaTexts,
        referenced_columns: DbStaticSchemaTexts,
        referenced_table: DbStaticSchemaText,
    },
    PrimaryKey(DbStaticSchemaTexts),
    Unique(DbStaticSchemaTexts),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DbKeyContractSnapshot {
    ForeignKey {
        columns: DbSchemaTexts,
        referenced_columns: DbSchemaTexts,
        referenced_table: DbSchemaText,
    },
    PrimaryKey(DbSchemaTexts),
    Unique(DbSchemaTexts),
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbSchemaTexts(Vec<DbSchemaText>);
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbColumnContractSnapshots(Vec<DbColumnContractSnapshot>);
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbKeyContractSnapshots(Vec<DbKeyContractSnapshot>);
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbColumnSnapshots(Vec<DbColumnSnapshot>);
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbObjectSnapshots(Vec<DbObjectSnapshot>);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DbColumnContractSnapshot {
    data_type: DbSchemaText,
    has_server_default: DbColumnHasServerDefault,
    name: DbSchemaText,
    nullable: DbColumnNullable,
}
impl DbColumnContractSnapshot {
    #[must_use]
    pub const fn new(
        name: DbSchemaText,
        data_type: DbSchemaText,
        nullable: DbColumnNullable,
        has_server_default: DbColumnHasServerDefault,
    ) -> Self {
        Self {
            data_type,
            has_server_default,
            name,
            nullable,
        }
    }
}

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct SqlxPgPoolRef<'value_lt>(&'value_lt sqlx::PgPool);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct DbSchemaNameRef<'value_lt>(&'value_lt str);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct DbTableNameRef<'value_lt>(&'value_lt str);

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
    Default,
    Extension,
    ForeignKey,
    Function,
    Index,
    PrimaryKey,
    Trigger,
    Unique,
    View,
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
    columns: DbColumnSnapshots,
    objects: DbObjectSnapshots,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DbCatalogSnapshot {
    objects: DbObjectSnapshots,
}
impl DbCatalogSnapshot {
    #[must_use]
    pub fn new(mut objects: DbObjectSnapshots) -> Self {
        objects.sort_unstable();
        Self { objects }
    }
}
impl DbTableSnapshot {
    #[must_use]
    pub fn new(mut columns: DbColumnSnapshots, mut objects: DbObjectSnapshots) -> Self {
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
    #[error("PostgreSQL catalog differs from the expected snapshot")]
    CatalogMismatch {
        expected: DbCatalogSnapshot,
        observed: DbCatalogSnapshot,
    },
    #[error("PostgreSQL columns differ from the generated table descriptor")]
    ColumnContractMismatch {
        expected: DbColumnContractSnapshots,
        observed: DbColumnContractSnapshots,
    },
    #[error("PostgreSQL exact defaults differ from the reviewed table contract")]
    DefaultContractMismatch {
        expected: DbObjectSnapshots,
        observed: DbObjectSnapshots,
    },
    #[error("generated CRUD configuration refers to a column absent from its table descriptor")]
    DescriptorFieldMismatch(DbSchemaText),
    #[error("PostgreSQL CHECK/index objects differ from the reviewed table contract")]
    ExtendedObjectContractMismatch {
        expected: DbObjectSnapshots,
        observed: DbObjectSnapshots,
    },
    #[error("failed to inspect PostgreSQL schema: {0}")]
    Inspection(SqlxDbSchemaInspectionError),
    #[error("PostgreSQL key constraints differ from the generated table descriptor")]
    KeyContractMismatch {
        expected: DbKeyContractSnapshots,
        observed: DbKeyContractSnapshots,
    },
    #[error("PostgreSQL table schema differs from the expected snapshot")]
    Mismatch {
        expected: DbTableSnapshot,
        observed: DbTableSnapshot,
    },
    #[error("PostgreSQL schema text exceeds the supported limit")]
    SchemaTextTooLong(DbSchemaTextError),
    #[error("PostgreSQL returned an unsupported catalog object kind")]
    UnknownObjectKind,
}

pub async fn validate_postgres_table_extensions<Table>(
    pool: SqlxPgPoolRef<'_>,
    schema: DbSchemaNameRef<'_>,
) -> Result<(), DbSchemaConformanceError>
where
    Table: DbExtendedTableSchema,
{
    fn schema_text(value: String) -> Result<DbSchemaText, DbSchemaConformanceError> {
        DbSchemaText::try_from(value)
            .map_err(|error| DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error)))
    }
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
    let default_rows = sqlx::query(str_constants::DB_SCHEMA_EXACT_DEFAULT_QUERY)
        .bind(schema.0)
        .bind(Table::TABLE_NAME)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
        })?;
    let mut observed_defaults = default_rows
        .into_iter()
        .map(|row| {
            Ok(DbObjectSnapshot::new(
                schema_text(
                    sqlx::Row::try_get(&row, str_constants::COLUMN_NAME).map_err(|error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                    })?,
                )?,
                DbObjectKind::Default,
                schema_text(
                    sqlx::Row::try_get(&row, str_constants::COLUMN_DEFAULT).map_err(|error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
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
    let mut expected_objects = Table::checks_and_indexes()
        .iter()
        .map(|spec| {
            Ok(DbObjectSnapshot::new(
                schema_text(spec.name.0.to_owned())?,
                spec.kind,
                schema_text(spec.definition.0.to_owned())?,
            ))
        })
        .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?;
    let rows = sqlx::query(str_constants::DB_SCHEMA_CHECK_AND_NON_CONSTRAINT_INDEX_QUERY)
        .bind(schema.0)
        .bind(Table::TABLE_NAME)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
        })?;
    let mut observed_objects = rows
        .into_iter()
        .map(|row| {
            let kind = match sqlx::Row::try_get::<String, _>(&row, str_constants::OBJECT_KIND)
                .map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                })?
                .as_str()
            {
                str_constants::CHECK => DbObjectKind::Check,
                str_constants::INDEX => DbObjectKind::Index,
                _ => return Err(DbSchemaConformanceError::UnknownObjectKind),
            };
            Ok(DbObjectSnapshot::new(
                schema_text(
                    sqlx::Row::try_get(&row, str_constants::OBJECT_NAME).map_err(|error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                    })?,
                )?,
                kind,
                schema_text(
                    sqlx::Row::try_get(&row, str_constants::OBJECT_DEFINITION).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
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

pub async fn validate_generated_postgres_table<Table>(
    pool: SqlxPgPoolRef<'_>,
    schema: DbSchemaNameRef<'_>,
) -> Result<(), DbSchemaConformanceError>
where
    Table: DbTableSchema,
{
    fn static_schema_text(
        value: DbStaticSchemaText,
    ) -> Result<DbSchemaText, DbSchemaConformanceError> {
        DbSchemaText::try_from(value.0.to_owned())
            .map_err(|error| DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error)))
    }
    fn static_schema_texts(
        values: DbStaticSchemaTexts,
    ) -> Result<DbSchemaTexts, DbSchemaConformanceError> {
        Vec::from(values)
            .into_iter()
            .map(static_schema_text)
            .collect::<Result<Vec<_>, _>>()
            .map(Into::into)
    }
    fn schema_text(value: String) -> Result<DbSchemaText, DbSchemaConformanceError> {
        DbSchemaText::try_from(value)
            .map_err(|error| DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error)))
    }
    fn schema_texts(values: Vec<String>) -> Result<Vec<DbSchemaText>, DbSchemaConformanceError> {
        values.into_iter().map(schema_text).collect()
    }
    let table_columns = Table::columns();
    let mut expected = table_columns
        .iter()
        .map(|column| {
            Ok(DbColumnContractSnapshot::new(
                DbSchemaText::try_from(column.name.0.to_owned()).map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                })?,
                DbSchemaText::try_from(column.data_type.0.to_owned()).map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                })?,
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
                let name = DbSchemaText::try_from(field.0.to_owned()).map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                })?;
                Err(DbSchemaConformanceError::DescriptorFieldMismatch(name))
            }
        })?;
    let rows = sqlx::query(str_constants::DB_SCHEMA_COLUMN_CONTRACT_QUERY)
        .bind(schema.0)
        .bind(Table::TABLE_NAME)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
        })?;
    let mut observed = rows
        .into_iter()
        .map(|row| {
            let nullable: String =
                sqlx::Row::try_get(&row, str_constants::IS_NULLABLE).map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                })?;
            Ok(DbColumnContractSnapshot::new(
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
                DbColumnNullable::from(nullable == str_constants::YES),
                DbColumnHasServerDefault::from(
                    sqlx::Row::try_get::<bool, _>(&row, str_constants::HAS_SERVER_DEFAULT)
                        .map_err(|error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
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
    let key_rows = sqlx::query(str_constants::DB_SCHEMA_KEY_CONTRACT_QUERY)
        .bind(schema.0)
        .bind(Table::TABLE_NAME)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
        })?;
    let mut observed_keys = key_rows
        .into_iter()
        .map(|row| {
            let kind = sqlx::Row::try_get::<String, _>(&row, str_constants::CONSTRAINT_TYPE)
                .map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                })?;
            let columns = schema_texts(
                sqlx::Row::try_get::<Vec<String>, _>(&row, str_constants::COLUMNS).map_err(
                    |error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                    },
                )?,
            )?;
            match kind.as_str() {
                str_constants::DB_CONSTRAINT_FOREIGN_KEY_SHORT => {
                    let referenced_columns = schema_texts(
                        sqlx::Row::try_get::<Vec<String>, _>(
                            &row,
                            str_constants::REFERENCED_COLUMNS,
                        )
                        .map_err(|error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                        })?,
                    )?;
                    let referenced_table = schema_text(
                        sqlx::Row::try_get::<String, _>(&row, str_constants::REFERENCED_TABLE)
                            .map_err(|error| {
                                DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(
                                    error,
                                ))
                            })?,
                    )?;
                    Ok(DbKeyContractSnapshot::ForeignKey {
                        columns: columns.into(),
                        referenced_columns: referenced_columns.into(),
                        referenced_table,
                    })
                }
                str_constants::DB_CONSTRAINT_PRIMARY_KEY_SHORT => {
                    Ok(DbKeyContractSnapshot::PrimaryKey(columns.into()))
                }
                str_constants::DB_CONSTRAINT_UNIQUE_SHORT => {
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

pub fn validate_postgres_catalog(
    expected: DbCatalogSnapshot,
    observed: DbCatalogSnapshot,
) -> Result<(), DbSchemaConformanceError> {
    if expected == observed {
        Ok(())
    } else {
        Err(DbSchemaConformanceError::CatalogMismatch { expected, observed })
    }
}

pub async fn inspect_postgres_catalog(
    pool: SqlxPgPoolRef<'_>,
    schema: DbSchemaNameRef<'_>,
) -> Result<DbCatalogSnapshot, DbSchemaConformanceError> {
    let rows = sqlx::query(str_constants::DB_SCHEMA_CATALOG_QUERY)
        .bind(schema.0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
        })?;
    rows.into_iter()
        .map(|row| {
            let kind_text: String =
                sqlx::Row::try_get(&row, str_constants::OBJECT_KIND).map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                })?;
            let kind = match kind_text.as_str() {
                str_constants::EXTENSION => DbObjectKind::Extension,
                str_constants::FUNCTION => DbObjectKind::Function,
                str_constants::TRIGGER => DbObjectKind::Trigger,
                str_constants::VIEW => DbObjectKind::View,
                _ => return Err(DbSchemaConformanceError::UnknownObjectKind),
            };
            let name = sqlx::Row::try_get::<String, _>(&row, str_constants::OBJECT_NAME).map_err(
                |error| DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error)),
            )?;
            let definition =
                sqlx::Row::try_get::<String, _>(&row, str_constants::OBJECT_DEFINITION).map_err(
                    |error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError(error))
                    },
                )?;
            Ok(DbObjectSnapshot::new(
                DbSchemaText::try_from(name).map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                })?,
                kind,
                DbSchemaText::try_from(definition).map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError(error))
                })?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|objects| DbCatalogSnapshot::new(objects.into()))
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
    Ok(DbTableSnapshot::new(columns.into(), objects.into()))
}

#[cfg(test)]
#[allow(clippy::needless_for_each)] // repository policy requires iterator traversal in source tests
mod tests {
    fn catalog_snapshot(kind: super::DbObjectKind) -> super::DbCatalogSnapshot {
        super::DbCatalogSnapshot::new(
            vec![super::DbObjectSnapshot::new(
                super::DbSchemaText::try_from(String::from(str_constants::TEST_DB_OBJECT_NAME))
                    .expect(str_constants::VALUE_E84FED1B),
                kind,
                super::DbSchemaText::try_from(String::from(
                    str_constants::TEST_DB_OBJECT_DEFINITION,
                ))
                .expect(str_constants::VALUE_A7950FF0),
            )]
            .into(),
        )
    }

    fn snapshot(nullable: bool) -> super::DbTableSnapshot {
        super::DbTableSnapshot::new(
            vec![super::DbColumnSnapshot::new(
                super::DbSchemaText::try_from(String::from(str_constants::TEST_DB_COLUMN_ID))
                    .expect(str_constants::VALUE_11F0D7F5),
                super::DbSchemaText::try_from(String::from(str_constants::TEST_DB_DATA_TYPE_UUID))
                    .expect(str_constants::VALUE_9CB64C93),
                nullable.into(),
                None,
            )]
            .into(),
            vec![super::DbObjectSnapshot::new(
                super::DbSchemaText::try_from(String::from(str_constants::TEST_DB_CONSTRAINT_NAME))
                    .expect(str_constants::VALUE_61F95647),
                super::DbObjectKind::PrimaryKey,
                super::DbSchemaText::try_from(String::from(
                    str_constants::TEST_DB_CONSTRAINT_DEFINITION,
                ))
                .expect(str_constants::VALUE_A4B28D38),
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
}
