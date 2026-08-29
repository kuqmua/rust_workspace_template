#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminPageClientMode {
    Csr,
    CsrTableQuery,
    Ssr,
}
impl AdminPageClientMode {
    pub(super) fn supports_csr(self) -> crate::admin_bool::AdminBool {
        crate::admin_bool::AdminBool::from(matches!(self, Self::Csr | Self::CsrTableQuery))
    }
    pub(super) fn uses_table_query(self) -> crate::admin_bool::AdminBool {
        crate::admin_bool::AdminBool::from(matches!(self, Self::CsrTableQuery))
    }
}
