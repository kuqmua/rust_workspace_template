#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub struct HttpOpentelemetryHeaderMapRef<'headers_lt>(&'headers_lt http::HeaderMap);

impl<'headers_lt> HttpOpentelemetryHeaderMapRef<'headers_lt> {
    pub(crate) const fn get(self) -> &'headers_lt http::HeaderMap {
        self.0
    }
}
