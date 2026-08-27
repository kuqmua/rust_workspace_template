#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(crate) enum AdminTableQueryDirection {
    #[cfg(target_arch = "wasm32")]
    Csr(Option<server_admin_contract::domain_types::AdminText>),
    #[cfg(not(target_arch = "wasm32"))]
    Ssr(server_admin_contract::domain_types::AdminSortDirection),
}
