#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_as_ref_inner::AsRefInner,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct HttpAdminHeaderMapRef<'headers_lt>(&'headers_lt http::HeaderMap);

impl<'headers_lt> HttpAdminHeaderMapRef<'headers_lt> {
    pub(crate) const fn get(self) -> &'headers_lt http::HeaderMap {
        self.get_inner()
    }
}
