#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[must_use]
pub fn wrap_derive(
    v: ProcMacro2DeriveTokensRef<'_>,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    let tokens = v.0;
    quote::quote! {#[derive(#(#tokens),*)]}.into()
}

pub use super::proc_macro2_derive_tokens_ref::ProcMacro2DeriveTokensRef;
#[cfg(test)]
mod tests {
    #[test]
    fn derive_wrapper_preserves_order_and_empty_input() {
        let derives = [quote::quote!(Debug), quote::quote!(Clone)];
        let derive_refs = [&derives[0], &derives[1]];
        let actual = super::wrap_derive(super::ProcMacro2DeriveTokensRef::from(
            derive_refs.as_slice(),
        ));
        assert_eq!(
            actual.as_ref().to_string(),
            quote::quote!(#[derive(Debug, Clone)]).to_string()
        );
        let empty_refs: [&proc_macro2::TokenStream; 0] = [];
        let empty = super::wrap_derive(super::ProcMacro2DeriveTokensRef::from(
            empty_refs.as_slice(),
        ));
        assert_eq!(
            empty.as_ref().to_string(),
            quote::quote!(#[derive()]).to_string()
        );
    }
}
