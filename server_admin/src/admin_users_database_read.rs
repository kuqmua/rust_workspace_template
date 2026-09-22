#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::partial_pub_fields,
    reason = "database users read keeps declaration order aligned with the PostgreSQL table"
)]
#[derive(
    Clone,
    Copy,
    proc_macro_generate_pg_table_derive_generate_pg_table::GeneratePgTable,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "db_table_name": "users",
    "create_exclude_fields": ["password_hash", "must_change_password", "created_at", "updated_at"],
    "read_exclude_fields": ["password_hash"],
    "rule_prefix": "users",
    "read_page": {
        "search_columns": ["login", "display_name"],
        "response": "server_admin_contract::admin_data_table_view::AdminDataTableView",
        "enrich": "crate::enrich_users_database_read_page::enrich_users_database_read_page",
        "error": "crate::admin_users_database_read_page_error::AdminUsersDatabaseReadPageError"
    },
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[allow(
    dead_code,
    reason = "database users read declares generated API members exercised through routing"
)]
#[derive(proc_macro_getters::Getters)]
pub struct AdminUsersDatabaseRead {
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    login: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    display_name: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    password_hash: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    is_banned: pg_types_numeric::generate_pg_types_mod::BoolAsNonNullBool,
    #[generate_pg_table_db_default]
    must_change_password: pg_types_numeric::generate_pg_types_mod::BoolAsNonNullBool,
    #[generate_pg_table_db_default]
    created_at: pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    #[generate_pg_table_db_default]
    updated_at: pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}

#[allow(
    clippy::missing_fields_in_debug,
    reason = "database users read debug output intentionally omits every field to protect credentials"
)]
impl std::fmt::Debug for AdminUsersDatabaseRead {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(constants_str::ADMINUSERS)
            .finish_non_exhaustive()
    }
}
