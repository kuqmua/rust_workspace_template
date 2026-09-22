#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin access sessions keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_generate_pg_table_derive_generate_pg_table::GeneratePgTable,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "db_table_name": "access_sessions",
    "create_exclude_fields": ["created_at"],
    "read_exclude_fields": ["token_identifier_hash", "csrf_token_hash", "token_context_hash"],
    "db_foreign_keys": [
        {"columns": ["user_id"], "referenced_columns": ["id"], "referenced_table": "users"}
    ],
    "db_unique_keys": [["token_identifier_hash"]],
    "rule_prefix": "access_sessions",
    "read_page": {
        "search_columns": [],
        "response": "server_admin_contract::admin_data_table_view::AdminDataTableView",
        "enrich": "crate::enrich_access_sessions_read_page::enrich_access_sessions_read_page",
        "error": "crate::admin_access_sessions_read_page_error::AdminAccessSessionsReadPageError"
    },
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
pub struct AdminAccessSessions {
    #[generate_pg_table_primary_key]
    id: pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg,
    user_id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8,
    token_identifier_hash: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    csrf_token_hash: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    token_context_hash: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    expires_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    #[generate_pg_table_db_default]
    created_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    revoked_at:
        pg_types_chrono_net::generate_pg_types_mod::OptionalSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableTimestampTz,
}
