#[must_use]
pub fn wrap_derive<ValueTokenStream>(value: &[&ValueTokenStream]) -> proc_macro2::TokenStream
where
    ValueTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! { #[derive(#(#value),*)] }
}
