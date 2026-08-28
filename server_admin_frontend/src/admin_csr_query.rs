#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub(crate) struct AdminCsrQuery {
    pub(crate) direction: Option<server_admin_contract::domain_types::AdminText>,
    pub(crate) filter_end: Option<server_admin_contract::domain_types::AdminFilterValue>,
    pub(crate) filter_field: Option<server_admin_contract::domain_types::AdminFilterField>,
    pub(crate) filter_operation:
        Option<server_admin_contract::domain_types::AdminFilterOperationKey>,
    pub(crate) filter_value: Option<server_admin_contract::domain_types::AdminFilterValue>,
    pub(crate) limit: server_admin_contract::domain_types::AdminPageLimit,
    pub(crate) offset: server_admin_contract::domain_types::AdminPageOffset,
    pub(crate) search: server_admin_contract::domain_types::AdminTableSearch,
    pub(crate) sort: server_admin_contract::domain_types::AdminTableSortKey,
    pub(crate) table: Option<server_admin_contract::domain_types::AdminDataTable>,
}

// Root-owned module compatibility wrappers.
pub(crate) mod csr_page_from_location {
    pub use crate::csr_page_from_location::*;
}
pub(crate) mod location {
    pub use crate::location::*;
}
