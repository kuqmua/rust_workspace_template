#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, newtype::FromInner)]
pub(super) struct ProcMacro2CaseTokenStream(pub(super) proc_macro2::TokenStream);
