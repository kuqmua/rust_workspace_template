#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::ToErrString,
    proc_macro_newtype::FromInner,
)]
pub struct AxumBodySizeError(axum::Error);
