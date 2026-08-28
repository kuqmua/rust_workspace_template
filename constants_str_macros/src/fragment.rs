#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct Fragment {
    pub(super) name: super::SynIdent,
    pub(super) value: super::SynLitStr,
}
