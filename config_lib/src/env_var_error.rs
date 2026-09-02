#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    PartialEq,
    Eq,
    thiserror::Error,
    proc_macro_newtype::FromInner,
)]
#[error(transparent)]
pub(super) struct EnvVarError(std::env::VarError);
