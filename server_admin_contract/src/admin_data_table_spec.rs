#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq,
)]
pub struct AdminDataTableSpec {
    #[getters(copy)]
    columns: crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef<'static>,
    #[getters(copy)]
    order: crate::admin_data_order_ref::AdminDataOrderRef<'static>,
    #[getters(copy)]
    permission: crate::admin_permission::AdminPermission,
    #[getters(copy)]
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
}
