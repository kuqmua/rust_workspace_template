#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(crate) struct ExpectedFileContentRef<'content_lt>(&'content_lt str);

impl<'content_lt> From<&'content_lt String> for ExpectedFileContentRef<'content_lt> {
    fn from(value: &'content_lt String) -> Self {
        Self(value.as_str())
    }
}
