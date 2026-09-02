#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    thiserror::Error,
    proc_macro_newtype::FromInner,
)]
#[error(transparent)]
pub struct SerdeJsonError(serde_json::Error);
