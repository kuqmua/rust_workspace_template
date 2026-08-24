#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct ProcMacro2DeriveTokensRef<'tokens_lt>(
    &'tokens_lt [&'tokens_lt proc_macro2::TokenStream],
);
#[must_use]
pub fn wrap_derive(
    v: ProcMacro2DeriveTokensRef<'_>,
) -> crate::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let tokens = v.0;
    quote::quote! {#[derive(#(#tokens),*)]}.into()
}

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
