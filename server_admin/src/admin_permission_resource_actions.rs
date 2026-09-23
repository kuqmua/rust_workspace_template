#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin permission resource actions keeps declaration order aligned with generated layout or processing flow"
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
    "db_table_name": "permission_resource_actions",
    "route_resource_name": "permission_resource_actions",
    "db_foreign_keys": [
        {"columns": ["permission_resource_id"], "referenced_columns": ["id"], "referenced_table": "permission_resources"},
        {"columns": ["permission_action_id"], "referenced_columns": ["id"], "referenced_table": "permission_actions"}
    ],
    "db_unique_keys": [["permission_resource_id", "permission_action_id"]],
    "rule_prefix": "permission_resource_actions",
    "read_page": {
        "search_columns": [],
        "response": "server_admin_contract::admin_data_table_view::AdminDataTableView",
        "enrich": "crate::enrich_permission_resource_actions_read_page::enrich_permission_resource_actions_read_page",
        "error": "crate::admin_permission_resource_actions_read_page_error::AdminPermissionResourceActionsReadPageError"
    },
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
pub struct AdminPermissionResourceActions {
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    permission_resource_id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8,
    permission_action_id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8,
}
