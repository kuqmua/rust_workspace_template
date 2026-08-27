#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct ExpectedFileContentRef<'content_lt>(pub(super) &'content_lt str);

impl<'content_lt> From<&'content_lt String> for ExpectedFileContentRef<'content_lt> {
    fn from(value: &'content_lt String) -> Self {
        Self(value.as_str())
    }
}
