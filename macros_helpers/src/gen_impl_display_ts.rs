#[must_use]
pub fn gen_impl_display_ts<
    ImplGenericsTokenStream,
    IdentifierTokenStream,
    IdentifierGenericsTokenStream,
    BodyTokenStream,
>(
    impl_generics: &ImplGenericsTokenStream,
    identifier: &IdentifierTokenStream,
    identifier_generics: &IdentifierGenericsTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    ImplGenericsTokenStream: quote::ToTokens + ?Sized,
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    IdentifierGenericsTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #impl_generics std::fmt::Display for #identifier #identifier_generics {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #body
            }
        }
    }
}
