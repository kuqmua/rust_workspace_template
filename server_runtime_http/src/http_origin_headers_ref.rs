#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpOriginHeadersRef<'header>(&'header http::HeaderMap);

impl<'header> HttpOriginHeadersRef<'header> {
    pub(crate) const fn get(self) -> &'header http::HeaderMap {
        self.0
    }
}
