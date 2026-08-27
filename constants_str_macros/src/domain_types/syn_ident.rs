#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct SynIdent(pub(super) syn::Ident);

impl From<syn::Ident> for SynIdent {
    fn from(value: syn::Ident) -> Self {
        Self(value)
    }
}

impl syn::parse::Parse for SynIdent {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}
