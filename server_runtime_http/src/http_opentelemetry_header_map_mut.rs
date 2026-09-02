#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::DerefMutInner,
    proc_macro_newtype::FromInner,
)]
pub struct HttpOpentelemetryHeaderMapMut<'headers_lt>(&'headers_lt mut http::HeaderMap);
