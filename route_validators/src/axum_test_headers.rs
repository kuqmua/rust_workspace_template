#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::DerefMutInner,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct AxumTestHeaders(axum::http::HeaderMap);
