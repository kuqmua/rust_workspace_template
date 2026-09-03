#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin roles keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_generate_pg_table::GeneratePgTable,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[proc_macro_generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "db_table_name": "roles",
    "create_exclude_fields": ["created_at", "updated_at"],
    "db_unique_keys": [["name"]],
    "permission_prefix": "roles",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
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
