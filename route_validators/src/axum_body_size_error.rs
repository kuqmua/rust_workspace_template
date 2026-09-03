#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_to_err_string::ToErrString,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct AxumBodySizeError(axum::Error);
