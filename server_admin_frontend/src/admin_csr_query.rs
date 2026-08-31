#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub(crate) struct AdminCsrQuery {
    direction: Option<server_admin_contract::admin_text::AdminText>,
    filter_end: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
    filter_field: Option<server_admin_contract::admin_filter_field::AdminFilterField>,
    filter_operation:
        Option<server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey>,
    filter_value: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
    limit: server_admin_contract::admin_page_limit::AdminPageLimit,
    offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    search: server_admin_contract::admin_table_search::AdminTableSearch,
    sort: server_admin_contract::admin_table_sort_key::AdminTableSortKey,
    table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
}

// Root-owned module compatibility wrappers.
pub(crate) mod csr_page_from_location {}
pub(crate) mod location {}
