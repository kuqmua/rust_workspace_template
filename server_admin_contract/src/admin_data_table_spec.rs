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
        admin_data_columns_csv_ref: crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef<
            'static,
        >,
        admin_data_order_ref: crate::admin_data_order_ref::AdminDataOrderRef<'static>,
        admin_permission: crate::admin_permission::AdminPermission,
        admin_bool: crate::admin_bool::AdminBool,
    ) -> Self {
        Self {
            columns: admin_data_columns_csv_ref,
            order: admin_data_order_ref,
            permission: admin_permission,
            supports_filters: admin_bool,
        }
    }
}
