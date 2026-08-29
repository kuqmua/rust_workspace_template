// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(
    Debug,
    Clone,
    Copy,
    generate_pg_table::GeneratePgTable,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "db_table_name": "permissions",
    "create_exclude_fields": ["created_at"],
    "db_unique_keys": [["name"]],
    "permission_prefix": "permissions",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
pub struct AdminPermissions {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    pub name: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub created_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
