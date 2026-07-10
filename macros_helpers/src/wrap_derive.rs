#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub struct ProcMacro2DeriveTokensRef<'tokens_lt>(
    &'tokens_lt [&'tokens_lt proc_macro2::TokenStream],
);
#[must_use]
pub fn wrap_derive(v: ProcMacro2DeriveTokensRef<'_>) -> crate::generated_rust_ts::GeneratedRustTs {
    let tokens = v.0;
    quote::quote! {#[derive(#(#tokens),*)]}.into()
}
