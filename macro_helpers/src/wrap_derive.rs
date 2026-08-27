#[path = "wrap_derive/proc_macro2_derive_tokens_ref.rs"]
mod proc_macro2_derive_tokens_ref;
#[path = "wrap_derive/wrap_derive.rs"]
mod wrap_derive;

pub use proc_macro2_derive_tokens_ref::ProcMacro2DeriveTokensRef;
pub use wrap_derive::wrap_derive;

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
