#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MemusageValueRef<'lt>(pub(super) &'lt str);
impl<'lt> MemusageValueRef<'lt> {
    pub(crate) const fn get(self) -> &'lt str {
        self.0
    }
}
