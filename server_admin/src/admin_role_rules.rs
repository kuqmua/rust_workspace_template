#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin role rules keeps declaration order aligned with generated layout or processing flow"
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
    "db_table_name": "role_rules",
    "create_exclude_fields": ["created_at"],
    "db_foreign_keys": [
        {"columns": ["role_id"], "referenced_columns": ["id"], "referenced_table": "roles"},
        {"columns": ["rule_id"], "referenced_columns": ["id"], "referenced_table": "rules"}
    ],
    "db_unique_keys": [["role_id", "rule_id"]],
    "rule_prefix": "role_rules",
    "read_page": {
        "search_columns": [],
        "response": "server_admin_contract::admin_data_table_view::AdminDataTableView",
        "enrich": "crate::enrich_role_rules_read_page::enrich_role_rules_read_page",
        "error": "crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError"
    },
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
pub struct AdminRoleRules {
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    role_id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8,
    rule_id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8,
    #[generate_pg_table_db_default]
    created_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
