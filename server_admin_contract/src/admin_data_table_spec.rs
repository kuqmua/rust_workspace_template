#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminDataTableSpec {
    columns: crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef<'static>,
    order: crate::admin_data_order_ref::AdminDataOrderRef<'static>,
    permission: crate::admin_permission::AdminPermission,
    supports_filters: crate::admin_bool::AdminBool,
}

impl AdminDataTableSpec {
    pub(super) const fn new(
        columns: crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef<'static>,
        order: crate::admin_data_order_ref::AdminDataOrderRef<'static>,
        permission: crate::admin_permission::AdminPermission,
        supports_filters: crate::admin_bool::AdminBool,
    ) -> Self {
        Self {
            columns,
            order,
            permission,
            supports_filters,
        }
    }
    #[must_use]
    pub const fn columns(
        self,
    ) -> crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef<'static> {
        self.columns
    }
    #[must_use]
    pub const fn order(self) -> crate::admin_data_order_ref::AdminDataOrderRef<'static> {
        self.order
    }
    #[must_use]
    pub const fn permission(self) -> crate::admin_permission::AdminPermission {
        self.permission
    }
    #[must_use]
    pub const fn supports_filters(self) -> crate::admin_bool::AdminBool {
        self.supports_filters
    }
}
