#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    thiserror::Error,
    proc_macro_newtype_from_inner::FromInner,
)]
#[error("{0}")]
pub struct ServiceRuntimeIoError(std::io::Error);
