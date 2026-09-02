#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
)]
pub struct HttpHeaderName(http::HeaderName);
