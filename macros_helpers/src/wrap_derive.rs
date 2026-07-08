#[derive(Debug, Clone, Copy)]
pub struct DeriveTokensRef<'tokens_lt>(&'tokens_lt [&'tokens_lt proc_macro2::TokenStream]);
impl<'tokens_lt> From<&'tokens_lt [&'tokens_lt proc_macro2::TokenStream]>
    for DeriveTokensRef<'tokens_lt>
{
    fn from(value: &'tokens_lt [&'tokens_lt proc_macro2::TokenStream]) -> Self {
        Self(value)
    }
}
#[must_use]
pub fn wrap_derive(v: DeriveTokensRef<'_>) -> crate::GeneratedRustTs {
    let tokens = v.0;
    quote::quote! {#[derive(#(#tokens),*)]}.into()
}
