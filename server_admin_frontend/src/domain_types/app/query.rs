pub(in crate::domain_types::app) mod page;

mod location;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub(in crate::domain_types::app) struct AdminCsrQuery {
    pub(in crate::domain_types::app) direction:
        Option<server_admin_contract::domain_types::AdminText>,
    pub(in crate::domain_types::app) filter_end:
        Option<server_admin_contract::domain_types::AdminFilterValue>,
    pub(in crate::domain_types::app) filter_field:
        Option<server_admin_contract::domain_types::AdminFilterField>,
    pub(in crate::domain_types::app) filter_operation:
        Option<server_admin_contract::domain_types::AdminFilterOperationKey>,
    pub(in crate::domain_types::app) filter_value:
        Option<server_admin_contract::domain_types::AdminFilterValue>,
    pub(in crate::domain_types::app) limit: server_admin_contract::domain_types::AdminPageLimit,
    pub(in crate::domain_types::app) offset: server_admin_contract::domain_types::AdminPageOffset,
    pub(in crate::domain_types::app) search: server_admin_contract::domain_types::AdminTableSearch,
    pub(in crate::domain_types::app) sort: server_admin_contract::domain_types::AdminTableSortKey,
    pub(in crate::domain_types::app) table:
        Option<server_admin_contract::domain_types::AdminDataTable>,
}
