#![allow(
    clippy::redundant_pattern_matching,
    reason = "the generated single-value lookup table update model triggers this lint inside proc-macro output"
)]

#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin permission actions keeps declaration order aligned with generated layout or processing flow"
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
    "db_table_name": "permission_actions",
    "db_unique_keys": [["key"]],
    "rule_prefix": "permission_actions",
    "read_page": {
        "search_columns": [],
        "response": "server_admin_contract::admin_data_table_view::AdminDataTableView",
        "enrich": "crate::enrich_permission_actions_read_page::enrich_permission_actions_read_page",
        "error": "crate::admin_permission_actions_read_page_error::AdminPermissionActionsReadPageError"
    },
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
pub struct AdminPermissionActions {
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    key: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
}
