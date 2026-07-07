#[derive(Debug, Clone, Copy)]
pub struct DeriveTokensRef<'tokens_lt>(pub &'tokens_lt [&'tokens_lt proc_macro2::TokenStream]);
#[must_use]
pub fn wrap_derive(v: DeriveTokensRef<'_>) -> crate::GeneratedRustTs {
    let tokens = v.0;
    quote::quote! {#[derive(#(#tokens),*)]}.into()
}
