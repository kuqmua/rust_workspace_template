#[derive(
    Debug,
    thiserror::Error,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[error(transparent)]
pub struct StdToolIoError(std::io::Error);
