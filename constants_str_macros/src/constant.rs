#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct Constant {
    pub(super) name: super::SynIdent,
    pub(super) parts: super::ConstantParts,
    pub(super) visibility: Option<super::SynVisibility>,
}
