//! PostgreSQL migrations are the runtime authority for database structure.
//! Generated Rust table descriptors are compile-time expectations and must be
//! checked against a database with all migrations applied before deployment.

#[path = "db_catalog_snapshot.rs"]
mod db_catalog_snapshot;
#[path = "db_column_contract_snapshot.rs"]
mod db_column_contract_snapshot;
#[path = "db_column_contract_snapshots.rs"]
mod db_column_contract_snapshots;
#[path = "db_column_has_server_default.rs"]
mod db_column_has_server_default;
#[path = "db_column_nullable.rs"]
mod db_column_nullable;
#[path = "db_column_snapshot.rs"]
mod db_column_snapshot;
#[path = "db_column_snapshots.rs"]
mod db_column_snapshots;
#[path = "db_column_spec.rs"]
mod db_column_spec;
#[path = "db_column_specs.rs"]
mod db_column_specs;
#[path = "db_default_spec.rs"]
mod db_default_spec;
#[path = "db_default_specs.rs"]
mod db_default_specs;
#[path = "db_extended_table_schema.rs"]
mod db_extended_table_schema;
#[path = "db_key_contract_snapshot.rs"]
mod db_key_contract_snapshot;
#[path = "db_key_contract_snapshots.rs"]
mod db_key_contract_snapshots;
#[path = "db_key_spec.rs"]
mod db_key_spec;
#[path = "db_key_specs.rs"]
mod db_key_specs;
#[path = "db_object_kind.rs"]
mod db_object_kind;
#[path = "db_object_snapshot.rs"]
mod db_object_snapshot;
#[path = "db_object_snapshots.rs"]
mod db_object_snapshots;
#[path = "db_object_spec.rs"]
mod db_object_spec;
#[path = "db_object_specs.rs"]
mod db_object_specs;
#[path = "db_schema_conformance_error.rs"]
mod db_schema_conformance_error;
#[path = "db_schema_name_ref.rs"]
mod db_schema_name_ref;
#[path = "db_schema_text.rs"]
mod db_schema_text;
#[path = "db_schema_text_error.rs"]
mod db_schema_text_error;
#[path = "db_schema_texts.rs"]
mod db_schema_texts;
#[path = "db_static_schema_text.rs"]
mod db_static_schema_text;
#[path = "db_static_schema_texts.rs"]
mod db_static_schema_texts;
#[path = "db_table_name_ref.rs"]
mod db_table_name_ref;
#[path = "db_table_schema.rs"]
mod db_table_schema;
#[path = "db_table_snapshot.rs"]
mod db_table_snapshot;
#[path = "inspect_postgres_catalog.rs"]
mod inspect_postgres_catalog;
#[path = "inspect_postgres_table.rs"]
mod inspect_postgres_table;
#[path = "pg_column_schema.rs"]
mod pg_column_schema;
#[path = "schema_text.rs"]
mod schema_text;
#[path = "schema_texts.rs"]
mod schema_texts;
#[path = "sqlx_db_schema_inspection_error.rs"]
mod sqlx_db_schema_inspection_error;
#[path = "sqlx_pg_pool_ref.rs"]
mod sqlx_pg_pool_ref;
#[path = "static_schema_text.rs"]
mod static_schema_text;
#[path = "static_schema_texts.rs"]
mod static_schema_texts;
#[path = "validate_generated_postgres_table.rs"]
mod validate_generated_postgres_table;
#[path = "validate_postgres_catalog.rs"]
mod validate_postgres_catalog;
#[path = "validate_postgres_table_extensions.rs"]
mod validate_postgres_table_extensions;
#[path = "validate_postgres_table_schema.rs"]
mod validate_postgres_table_schema;
#[path = "validate_snapshot.rs"]
mod validate_snapshot;

pub use db_catalog_snapshot::DbCatalogSnapshot;
pub use db_column_contract_snapshot::DbColumnContractSnapshot;
pub use db_column_contract_snapshots::DbColumnContractSnapshots;
pub use db_column_has_server_default::DbColumnHasServerDefault;
pub use db_column_nullable::DbColumnNullable;
pub use db_column_snapshot::DbColumnSnapshot;
pub use db_column_snapshots::DbColumnSnapshots;
pub use db_column_spec::DbColumnSpec;
pub use db_column_specs::DbColumnSpecs;
pub use db_default_spec::DbDefaultSpec;
pub use db_default_specs::DbDefaultSpecs;
pub use db_extended_table_schema::DbExtendedTableSchema;
pub use db_key_contract_snapshot::DbKeyContractSnapshot;
pub use db_key_contract_snapshots::DbKeyContractSnapshots;
pub use db_key_spec::DbKeySpec;
pub use db_key_specs::DbKeySpecs;
pub use db_object_kind::DbObjectKind;
pub use db_object_snapshot::DbObjectSnapshot;
pub use db_object_snapshots::DbObjectSnapshots;
pub use db_object_spec::DbObjectSpec;
pub use db_object_specs::DbObjectSpecs;
pub use db_schema_conformance_error::DbSchemaConformanceError;
pub use db_schema_name_ref::DbSchemaNameRef;
pub use db_schema_text::{DbSchemaText, DbSchemaTextTryFromStringError};
pub use db_schema_text_error::DbSchemaTextError;
pub use db_schema_texts::DbSchemaTexts;
pub use db_static_schema_text::DbStaticSchemaText;
pub use db_static_schema_texts::DbStaticSchemaTexts;
pub use db_table_name_ref::DbTableNameRef;
pub use db_table_schema::DbTableSchema;
pub use db_table_snapshot::DbTableSnapshot;
pub use inspect_postgres_catalog::inspect_postgres_catalog;
pub use inspect_postgres_table::inspect_postgres_table;
pub use pg_column_schema::PgColumnSchema;
pub(super) use schema_text::schema_text;
pub(super) use schema_texts::schema_texts;
pub use sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError;
pub use sqlx_pg_pool_ref::SqlxPgPoolRef;
pub(super) use static_schema_text::static_schema_text;
pub(super) use static_schema_texts::static_schema_texts;
pub use validate_generated_postgres_table::validate_generated_postgres_table;
pub use validate_postgres_catalog::validate_postgres_catalog;
pub use validate_postgres_table_extensions::validate_postgres_table_extensions;
pub use validate_postgres_table_schema::validate_postgres_table_schema;
pub(super) use validate_snapshot::validate_snapshot;

#[cfg(test)]
#[path = "domain_types_db_schema_conformance_tests.rs"]
mod tests;
