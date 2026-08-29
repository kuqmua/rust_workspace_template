#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct StdAssertFilePath<'path_lt>(pub(super) &'path_lt std::path::Path);

impl<'path_lt> StdAssertFilePath<'path_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<crate::assert_file_path_ref::AssertFilePathRef<'path_lt>>,
    {
        Self::from(v.into().0)
    }
}
