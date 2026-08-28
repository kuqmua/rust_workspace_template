#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct ServerAdminCleanupCfgError(server_admin::domain_types::AdminCleanupCfgError);
