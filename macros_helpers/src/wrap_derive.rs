#[must_use]
pub fn wrap_derive(v: &[&proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    quote::quote! {#[derive(#(#v),*)]}
}
