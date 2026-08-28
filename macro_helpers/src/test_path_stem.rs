#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct TestPathStem<'stem_lt>(pub(super) &'stem_lt str);

impl TestPathStem<'_> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<Self>,
    {
        v.into()
    }
}
