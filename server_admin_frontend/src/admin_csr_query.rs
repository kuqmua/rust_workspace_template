#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
)]
#[getters(bare)]
pub(crate) struct AdminCsrQuery {
    direction: Option<server_admin_contract::admin_text::AdminText>,
    filter_end: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
    filter_field: Option<server_admin_contract::admin_filter_field::AdminFilterField>,
    filter_operation:
        Option<server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey>,
    filter_value: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
    #[getters(copy)]
    limit: server_admin_contract::admin_page_limit::AdminPageLimit,
    #[getters(copy)]
    offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    search: server_admin_contract::admin_table_search::AdminTableSearch,
    sort: server_admin_contract::admin_table_sort_key::AdminTableSortKey,
    #[getters(copy)]
    table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
}

impl AdminCsrQuery {
    #[allow(
        clippy::too_many_arguments,
        reason = "the constructor validates ownership of every private query field at the module boundary"
    )]
    pub(crate) const fn new(
        direction: Option<server_admin_contract::admin_text::AdminText>,
        filter_end: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
        filter_field: Option<server_admin_contract::admin_filter_field::AdminFilterField>,
        filter_operation: Option<
            server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey,
        >,
        filter_value: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
        limit: server_admin_contract::admin_page_limit::AdminPageLimit,
        offset: server_admin_contract::admin_page_offset::AdminPageOffset,
        search: server_admin_contract::admin_table_search::AdminTableSearch,
        sort: server_admin_contract::admin_table_sort_key::AdminTableSortKey,
        table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
    ) -> Self {
        Self {
            direction,
            filter_end,
            filter_field,
            filter_operation,
            filter_value,
            limit,
            offset,
            search,
            sort,
            table,
        }
    }
}
