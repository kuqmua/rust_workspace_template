//! PostgreSQL migrations are the runtime authority for database structure.
//! Generated Rust table descriptors are compile-time expectations and must be
//! checked against a database with all migrations applied before deployment.

const DB_SCHEMA_TEXT_MAX_LEN: usize = 1_048_576usize;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::BoundedString,
)]
#[bounded_string(max = DB_SCHEMA_TEXT_MAX_LEN)]
pub struct DbSchemaText(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::Display,
    newtype::FromInner,
)]
pub struct DbSchemaTextError(DbSchemaTextTryFromStringError);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::FromInner,
)]
pub struct DbColumnNullable(bool);

pub trait PgColumnSchema {
    const HAS_SERVER_DEFAULT: bool;
    const NULLABLE: bool;
    fn data_type() -> DbStaticSchemaText;
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct DbStaticSchemaText(&'static str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::FromInner,
)]
pub struct DbColumnHasServerDefault(bool);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct DbColumnSpec {
    data_type: DbStaticSchemaText,
    name: DbStaticSchemaText,
    has_server_default: DbColumnHasServerDefault,
    nullable: DbColumnNullable,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
            name,
            has_server_default,
            nullable,
        }
    }
}

pub trait DbTableSchema {
    fn columns() -> DbColumnSpecs;
    fn create_excluded_columns() -> DbStaticSchemaTexts;
    fn keys() -> DbKeySpecs;
    fn primary_key_column() -> DbStaticSchemaText;
    fn read_excluded_columns() -> DbStaticSchemaTexts;
    fn schema_table_text() -> DbStaticSchemaText;
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct DbObjectSpec {
    definition: DbStaticSchemaText,
    name: DbStaticSchemaText,
    kind: DbObjectKind,
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
            name,
            kind,
        }
    }
}
pub trait DbExtendedTableSchema: DbTableSchema {
    fn checks_and_indexes() -> DbObjectSpecs;
    fn exact_defaults() -> DbDefaultSpecs;
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum DbKeySpec {
    ForeignKey {
        columns: DbStaticSchemaTexts,
        referenced_columns: DbStaticSchemaTexts,
        referenced_table: DbStaticSchemaText,
    },
    PrimaryKey(DbStaticSchemaTexts),
    Unique(DbStaticSchemaTexts),
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct DbColumnContractSnapshot {
    data_type: DbSchemaText,
    name: DbSchemaText,
    has_server_default: DbColumnHasServerDefault,
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
            name,
            has_server_default,
            nullable,
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct SqlxPgPoolRef<'value_lt>(&'value_lt sqlx::PgPool);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct DbSchemaNameRef<'value_lt>(&'value_lt str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct DbTableNameRef<'value_lt>(&'value_lt str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
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

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
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

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct DbObjectSnapshot {
    definition: DbSchemaText,
    name: DbSchemaText,
    kind: DbObjectKind,
}
impl DbObjectSnapshot {
    #[must_use]
    pub const fn new(name: DbSchemaText, kind: DbObjectKind, definition: DbSchemaText) -> Self {
        Self {
            definition,
            name,
            kind,
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DbTableSnapshot {
    columns: DbColumnSnapshots,
    objects: DbObjectSnapshots,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
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

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub struct SqlxDbSchemaInspectionError(sqlx::Error);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
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
        DbSchemaText::try_from(value).map_err(|error| {
            DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
        })
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
                    sqlx::Row::try_get(&row, str_constants::COLUMN_NAME).map_err(|error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                            error,
                        ))
                    })?,
                )?,
                DbObjectKind::Default,
                schema_text(
                    sqlx::Row::try_get(&row, str_constants::COLUMN_DEFAULT).map_err(|error| {
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
    let public_schema_qualifier = format!("{}.", str_constants::PUBLIC);
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
    let rows = sqlx::query(str_constants::DB_SCHEMA_CHECK_AND_NON_CONSTRAINT_INDEX_QUERY)
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
            let kind = match sqlx::Row::try_get::<String, _>(&row, str_constants::OBJECT_KIND)
                .map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
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
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                            error,
                        ))
                    })?,
                )?,
                kind,
                schema_text(
                    sqlx::Row::try_get(&row, str_constants::OBJECT_DEFINITION).map_err(
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
        DbSchemaText::try_from(value.0.to_owned()).map_err(|error| {
            DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
        })
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
        DbSchemaText::try_from(value).map_err(|error| {
            DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
        })
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
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                })?,
                DbSchemaText::try_from(column.data_type.0.to_owned()).map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
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
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                })?;
                Err(DbSchemaConformanceError::DescriptorFieldMismatch(name))
            }
        })?;
    let rows = sqlx::query(str_constants::DB_SCHEMA_COLUMN_CONTRACT_QUERY)
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
                sqlx::Row::try_get(&row, str_constants::IS_NULLABLE).map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                })?;
            Ok(DbColumnContractSnapshot::new(
                DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, str_constants::COLUMN_NAME).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                })?,
                DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, str_constants::DATA_TYPE).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                })?,
                DbColumnNullable::from(nullable == str_constants::YES),
                DbColumnHasServerDefault::from(
                    sqlx::Row::try_get::<bool, _>(&row, str_constants::HAS_SERVER_DEFAULT)
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
    let key_rows = sqlx::query(str_constants::DB_SCHEMA_KEY_CONTRACT_QUERY)
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
            let kind = sqlx::Row::try_get::<String, _>(&row, str_constants::CONSTRAINT_TYPE)
                .map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                })?;
            let columns = schema_texts(
                sqlx::Row::try_get::<Vec<String>, _>(&row, str_constants::COLUMNS).map_err(
                    |error| {
                        DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                            error,
                        ))
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
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        })?,
                    )?;
                    let referenced_table = schema_text(
                        sqlx::Row::try_get::<String, _>(&row, str_constants::REFERENCED_TABLE)
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
    validate_snapshot(
        expected,
        observed,
        |expected_snapshot, observed_snapshot| DbSchemaConformanceError::Mismatch {
            expected: expected_snapshot,
            observed: observed_snapshot,
        },
    )
}

pub fn validate_postgres_catalog(
    expected: DbCatalogSnapshot,
    observed: DbCatalogSnapshot,
) -> Result<(), DbSchemaConformanceError> {
    validate_snapshot(
        expected,
        observed,
        |expected_snapshot, observed_snapshot| DbSchemaConformanceError::CatalogMismatch {
            expected: expected_snapshot,
            observed: observed_snapshot,
        },
    )
}

fn validate_snapshot<Snapshot, Error, Mismatch>(
    expected: Snapshot,
    observed: Snapshot,
    mismatch: Mismatch,
) -> Result<(), Error>
where
    Snapshot: PartialEq,
    Mismatch: FnOnce(Snapshot, Snapshot) -> Error,
{
    if expected == observed {
        Ok(())
    } else {
        Err(mismatch(expected, observed))
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
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    rows.into_iter()
        .map(|row| {
            let kind_text: String =
                sqlx::Row::try_get(&row, str_constants::OBJECT_KIND).map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                })?;
            let kind = match kind_text.as_str() {
                str_constants::EXTENSION => DbObjectKind::Extension,
                str_constants::FUNCTION => DbObjectKind::Function,
                str_constants::TRIGGER => DbObjectKind::Trigger,
                str_constants::VIEW => DbObjectKind::View,
                _ => return Err(DbSchemaConformanceError::UnknownObjectKind),
            };
            let name = sqlx::Row::try_get::<String, _>(&row, str_constants::OBJECT_NAME).map_err(
                |error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                },
            )?;
            let definition = sqlx::Row::try_get::<String, _>(
                &row,
                str_constants::OBJECT_DEFINITION,
            )
            .map_err(|error| {
                DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
            })?;
            Ok(DbObjectSnapshot::new(
                DbSchemaText::try_from(name).map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                })?,
                kind,
                DbSchemaText::try_from(definition).map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
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
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    let columns = column_rows
        .into_iter()
        .map(|row| {
            let nullable: String =
                sqlx::Row::try_get(&row, str_constants::IS_NULLABLE).map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                })?;
            let default: Option<String> = sqlx::Row::try_get(&row, str_constants::COLUMN_DEFAULT)
                .map_err(|error| {
                DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
            })?;
            Ok(DbColumnSnapshot::new(
                DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, str_constants::COLUMN_NAME).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                })?,
                DbSchemaText::try_from(
                    sqlx::Row::try_get::<String, _>(&row, str_constants::DATA_TYPE).map_err(
                        |error| {
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                })?,
                (nullable == str_constants::YES).into(),
                default
                    .map(DbSchemaText::try_from)
                    .transpose()
                    .map_err(|error| {
                        DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
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
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    let mut objects = constraint_rows
        .into_iter()
        .map(|row| {
            let constraint_type: String = sqlx::Row::try_get(&row, str_constants::CONSTRAINT_TYPE)
                .map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
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
                            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(
                                error,
                            ))
                        },
                    )?,
                )
                .map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                })
                .and_then(|name| {
                    DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, str_constants::CONSTRAINT_DEFINITION)
                            .map_err(|error| {
                                DbSchemaConformanceError::Inspection(
                                    SqlxDbSchemaInspectionError::from(error),
                                )
                            })?,
                    )
                    .map_err(|error| {
                        DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
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
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    objects.extend(
        index_rows
            .into_iter()
            .map(|row| {
                Ok(DbObjectSnapshot::new(
                    DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, str_constants::INDEX_NAME).map_err(
                            |error| {
                                DbSchemaConformanceError::Inspection(
                                    SqlxDbSchemaInspectionError::from(error),
                                )
                            },
                        )?,
                    )
                    .map_err(|error| {
                        DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                    })?,
                    DbObjectKind::Index,
                    DbSchemaText::try_from(
                        sqlx::Row::try_get::<String, _>(&row, str_constants::INDEX_DEFINITION)
                            .map_err(|error| {
                                DbSchemaConformanceError::Inspection(
                                    SqlxDbSchemaInspectionError::from(error),
                                )
                            })?,
                    )
                    .map_err(|error| {
                        DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                    })?,
                ))
            })
            .collect::<Result<Vec<_>, DbSchemaConformanceError>>()?,
    );
    Ok(DbTableSnapshot::new(columns.into(), objects.into()))
}

#[cfg(test)]
mod tests;
