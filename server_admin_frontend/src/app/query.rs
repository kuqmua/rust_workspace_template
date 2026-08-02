pub(in crate::app) mod page;

mod location;

#[derive(optml::Optml, Clone, Debug, Default)]
pub(in crate::app) struct AdminCsrQuery {
    pub(in crate::app) direction: Option<server_admin_contract::AdminText>,
    pub(in crate::app) filter_end: Option<server_admin_contract::AdminFilterValue>,
    pub(in crate::app) filter_field: Option<server_admin_contract::AdminFilterField>,
    pub(in crate::app) filter_operation: Option<server_admin_contract::AdminFilterOperationKey>,
    pub(in crate::app) filter_value: Option<server_admin_contract::AdminFilterValue>,
    pub(in crate::app) limit: server_admin_contract::AdminPageLimit,
    pub(in crate::app) offset: server_admin_contract::AdminPageOffset,
    pub(in crate::app) search: server_admin_contract::AdminTableSearch,
    pub(in crate::app) sort: server_admin_contract::AdminTableSortKey,
    pub(in crate::app) table: Option<server_admin_contract::AdminDataTable>,
}
