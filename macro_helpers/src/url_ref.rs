#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct UrlRef<'url_lt>(&'url_lt str);

impl<'url_lt> UrlRef<'url_lt> {
    pub(crate) const fn as_str(self) -> &'url_lt str {
        self.0
    }
}
