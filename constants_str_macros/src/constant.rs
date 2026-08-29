#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct Constant {
    pub(super) name: crate::syn_ident::SynIdent,
    pub(super) parts: crate::constant_parts::ConstantParts,
    pub(super) visibility: Option<crate::syn_visibility::SynVisibility>,
}
