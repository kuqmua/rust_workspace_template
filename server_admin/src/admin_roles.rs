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
    "db_table_name": "roles",
    "create_exclude_fields": ["created_at", "updated_at"],
    "db_unique_keys": [["name"]],
    "permission_prefix": "roles",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(generate_accessor::Getters)]
pub struct AdminRoles {
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    name: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    is_system: pg_types_numeric::generate_pg_types_mod::BoolAsNonNullBool,
    #[generate_pg_table_db_default]
    created_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    #[generate_pg_table_db_default]
    updated_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
