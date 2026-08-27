#[must_use]
pub fn wrap_derive(
    v: super::ProcMacro2DeriveTokensRef<'_>,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    let tokens = v.0;
    quote::quote! {#[derive(#(#tokens),*)]}.into()
}
