#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct AssertFilePathRef<'path_lt>(pub(super) &'path_lt std::path::Path);

impl<'path_lt> From<&'path_lt std::path::PathBuf> for AssertFilePathRef<'path_lt> {
    fn from(value: &'path_lt std::path::PathBuf) -> Self {
        Self(value.as_path())
    }
}
