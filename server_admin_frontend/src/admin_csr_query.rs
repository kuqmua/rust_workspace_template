#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub(crate) struct AdminCsrQuery {
    pub(crate) direction: Option<server_admin_contract::admin_text::AdminText>,
    pub(crate) filter_end: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
    pub(crate) filter_field: Option<server_admin_contract::admin_filter_field::AdminFilterField>,
    pub(crate) filter_operation:
        Option<server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey>,
    pub(crate) filter_value: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
    pub(crate) limit: server_admin_contract::admin_page_limit::AdminPageLimit,
    pub(crate) offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    pub(crate) search: server_admin_contract::admin_table_search::AdminTableSearch,
    pub(crate) sort: server_admin_contract::admin_table_sort_key::AdminTableSortKey,
    pub(crate) table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
}

// Root-owned module compatibility wrappers.
pub(crate) mod csr_page_from_location {}
pub(crate) mod location {}
