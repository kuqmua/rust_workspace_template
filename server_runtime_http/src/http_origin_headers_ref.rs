#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct HttpOriginHeadersRef<'header>(&'header http::HeaderMap);

impl<'header> HttpOriginHeadersRef<'header> {
    pub(crate) const fn get(self) -> &'header http::HeaderMap {
        self.0
    }
}
