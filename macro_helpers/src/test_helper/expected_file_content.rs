#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct ExpectedFileContent<'content_lt>(pub(super) &'content_lt str);

impl<'content_lt> ExpectedFileContent<'content_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<super::ExpectedFileContentRef<'content_lt>>,
    {
        Self::from(v.into().0)
    }
}
