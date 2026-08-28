#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminDataTableSpec {
    columns: super::AdminDataColumnsCsvRef<'static>,
    order: super::AdminDataOrderRef<'static>,
    permission: super::AdminPermission,
    supports_filters: crate::domain_types::AdminBool,
}

impl AdminDataTableSpec {
    pub(super) const fn new(
        columns: super::AdminDataColumnsCsvRef<'static>,
        order: super::AdminDataOrderRef<'static>,
        permission: super::AdminPermission,
        supports_filters: crate::domain_types::AdminBool,
    ) -> Self {
        Self {
            columns,
            order,
            permission,
            supports_filters,
        }
    }
    #[must_use]
    pub const fn columns(self) -> super::AdminDataColumnsCsvRef<'static> {
        self.columns
    }
    #[must_use]
    pub const fn order(self) -> super::AdminDataOrderRef<'static> {
        self.order
    }
    #[must_use]
    pub const fn permission(self) -> super::AdminPermission {
        self.permission
    }
    #[must_use]
    pub const fn supports_filters(self) -> crate::domain_types::AdminBool {
        self.supports_filters
    }
}
