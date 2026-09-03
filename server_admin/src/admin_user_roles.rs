#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin user roles keeps declaration order aligned with generated layout or processing flow"
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
    "db_table_name": "user_roles",
    "create_exclude_fields": ["created_at"],
    "db_foreign_keys": [
        {"columns": ["user_id"], "referenced_columns": ["id"], "referenced_table": "users"},
        {"columns": ["role_id"], "referenced_columns": ["id"], "referenced_table": "roles"}
    ],
    "db_unique_keys": [["user_id", "role_id"]],
    "permission_prefix": "user_roles",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
pub struct AdminUserRoles {
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    user_id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8,
    role_id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8,
    #[generate_pg_table_db_default]
    created_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
