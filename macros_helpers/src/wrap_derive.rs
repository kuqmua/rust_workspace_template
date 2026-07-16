#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub struct ProcMacro2DeriveTokensRef<'tokens_lt>(
    &'tokens_lt [&'tokens_lt proc_macro2::TokenStream],
);
#[must_use]
pub fn wrap_derive(
    v: ProcMacro2DeriveTokensRef<'_>,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    let tokens = v.0;
    quote::quote! {#[derive(#(#tokens),*)]}.into()
}
