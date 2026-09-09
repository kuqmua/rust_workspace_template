#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    Clone,
    Debug,
    thiserror::Error,
)]
#[error(transparent)]
pub(crate) struct StdStrUtf8Error(std::str::Utf8Error);
