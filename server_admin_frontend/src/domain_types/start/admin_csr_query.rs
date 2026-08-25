pub(in crate::domain_types::start) mod csr_page_from_location;

mod location;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub(in crate::domain_types::start) struct AdminCsrQuery {
    pub(in crate::domain_types::start) direction:
        Option<server_admin_contract::domain_types::AdminText>,
    pub(in crate::domain_types::start) filter_end:
        Option<server_admin_contract::domain_types::AdminFilterValue>,
    pub(in crate::domain_types::start) filter_field:
        Option<server_admin_contract::domain_types::AdminFilterField>,
    pub(in crate::domain_types::start) filter_operation:
        Option<server_admin_contract::domain_types::AdminFilterOperationKey>,
    pub(in crate::domain_types::start) filter_value:
        Option<server_admin_contract::domain_types::AdminFilterValue>,
    pub(in crate::domain_types::start) limit: server_admin_contract::domain_types::AdminPageLimit,
    pub(in crate::domain_types::start) offset: server_admin_contract::domain_types::AdminPageOffset,
    pub(in crate::domain_types::start) search:
        server_admin_contract::domain_types::AdminTableSearch,
    pub(in crate::domain_types::start) sort: server_admin_contract::domain_types::AdminTableSortKey,
    pub(in crate::domain_types::start) table:
        Option<server_admin_contract::domain_types::AdminDataTable>,
}
