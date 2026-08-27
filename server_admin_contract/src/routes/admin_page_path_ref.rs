#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct AdminPagePathRef<'path_lt>(pub(super) &'path_lt str);
impl<'path_lt> AdminPagePathRef<'path_lt> {
    pub(crate) const fn get(self) -> &'path_lt str {
        self.0
    }
}
