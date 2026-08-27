#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ServerAdminAuthSvcStateBuildError(
    server_admin::domain_types::auth::AdminAuthSvcStateBuildError,
);
