#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct ProcMacro2DeriveTokensRef<'tokens_lt>(
    pub(super) &'tokens_lt [&'tokens_lt proc_macro2::TokenStream],
);
