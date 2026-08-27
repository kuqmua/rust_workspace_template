#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminPageClientMode {
    Csr,
    CsrTableQuery,
    Ssr,
}
impl AdminPageClientMode {
    pub(super) fn supports_csr(self) -> crate::domain_types::AdminBool {
        crate::domain_types::AdminBool::from(matches!(self, Self::Csr | Self::CsrTableQuery))
    }
    pub(super) fn uses_table_query(self) -> crate::domain_types::AdminBool {
        crate::domain_types::AdminBool::from(matches!(self, Self::CsrTableQuery))
    }
}
