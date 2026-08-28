#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    PartialEq,
    Eq,
    thiserror::Error,
    newtype::FromInner,
)]
#[error(transparent)]
pub(super) struct EnvVarError(std::env::VarError);
